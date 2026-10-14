//! Unified end-to-end analysis pipeline orchestration.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use rparse_cfg::{build_cfg, Cfg};
use rparse_core::{Diagnostic, DiagnosticSeverity, SourceFile, SourceId};
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions, ControlDependencies, DefUseInfo, PostDominators,
    ReachingDefinitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameAnalysis, DataFrameSeed};
use rparse_ir::{normalize_cst, Argument, Expr, ExtractKind, LValue, Program, Stmt};
use rparse_parser::RParser;
use rparse_pdg::{build_pdg, validate_pdg, ProgramDependenceGraph};
use rparse_runtime::bridge::RuntimeBridge;
use rparse_runtime::config::{AnalysisMode, RuntimeConfig};
use rparse_runtime::evidence::RuntimeEvidence;
use rparse_scope::{resolve_scopes, ScopeId, ScopeModel};

use crate::error::CliError;

/// Reusable end-to-end analysis context holding all static and runtime analysis models.
pub struct AnalysisContext {
    pub file_path: PathBuf,
    pub source_file: SourceFile,
    pub cst: rparse_parser::CstTree,
    pub program: Program,
    pub scope_model: ScopeModel,
    pub cfg: Cfg,
    pub def_use: DefUseInfo,
    pub reaching_defs: ReachingDefinitions,
    pub post_dominators: PostDominators,
    pub control_deps: ControlDependencies,
    pub df_analysis: DataFrameAnalysis,
    pub pdg: ProgramDependenceGraph,
    pub diagnostics: Vec<Diagnostic>,
    pub runtime_evidence: Vec<RuntimeEvidence>,
    pub mode: AnalysisMode,
}

impl AnalysisContext {
    /// Executes the full static pipeline on the specified source file (or stdin if file is "-").
    pub fn run(
        file: &Path,
        mode: AnalysisMode,
        seeds: Vec<DataFrameSeed>,
    ) -> Result<Self, CliError> {
        Self::run_with_reader(file, io::stdin(), mode, seeds)
    }

    /// Executes the full static pipeline on the specified source file, or from the provided reader if file is "-".
    pub fn run_with_reader<R: Read>(
        file: &Path,
        mut reader: R,
        mode: AnalysisMode,
        seeds: Vec<DataFrameSeed>,
    ) -> Result<Self, CliError> {
        let source_text = if file.as_os_str() == "-" {
            let mut buf = String::new();
            reader
                .read_to_string(&mut buf)
                .map_err(|e| CliError::InputFile {
                    path: file.to_path_buf(),
                    message: format!("Failed to read from stdin: {}", e),
                })?;
            buf
        } else {
            if !file.exists() {
                return Err(CliError::InputFile {
                    path: file.to_path_buf(),
                    message: "File does not exist".to_string(),
                });
            }

            if !file.is_file() {
                return Err(CliError::InputFile {
                    path: file.to_path_buf(),
                    message: "Path is a directory, expected an R script file".to_string(),
                });
            }

            fs::read_to_string(file).map_err(|e| CliError::InputFile {
                path: file.to_path_buf(),
                message: e.to_string(),
            })?
        };

        Self::run_from_source(file.to_path_buf(), source_text, mode, seeds)
    }

    /// Executes the full static pipeline on in-memory source code.
    pub fn run_from_source(
        file_path: PathBuf,
        source_text: String,
        mode: AnalysisMode,
        explicit_seeds: Vec<DataFrameSeed>,
    ) -> Result<Self, CliError> {
        let source_file = SourceFile::with_path(
            SourceId::new(0),
            file_path.to_string_lossy().to_string(),
            &file_path,
            source_text.clone(),
        );

        let mut parser = RParser::new().map_err(|e| CliError::Internal(e.to_string()))?;
        let cst_tree = parser
            .parse(&source_text)
            .map_err(|e| CliError::Internal(e.to_string()))?;
        let diagnostics = cst_tree.diagnostics();

        let program = normalize_cst(&cst_tree);
        let scope_model = resolve_scopes(&program);

        let cfg = build_cfg(&program).map_err(|e| CliError::AnalysisFailed(e.to_string()))?;
        let def_use = collect_def_use(&program, &scope_model, &cfg);
        let reaching_defs = compute_reaching_definitions(&cfg, &def_use);
        let post_dominators = compute_post_dominators(&cfg);
        let control_deps = compute_control_dependencies(&cfg, &post_dominators);

        let seeds = if !explicit_seeds.is_empty() {
            explicit_seeds
        } else {
            discover_seeds(&program, &scope_model)
        };

        let df_analysis = analyze_dataframes(&program, &scope_model, &seeds);

        let pdg = build_pdg(
            &program,
            &scope_model,
            &cfg,
            &def_use,
            &reaching_defs,
            &control_deps,
            &df_analysis,
        );

        if let Err(err) = validate_pdg(&pdg) {
            return Err(CliError::AnalysisFailed(format!(
                "PDG validation failed: {}",
                err
            )));
        }

        let mut runtime_evidence = Vec::new();

        // Optional runtime observation (only in Assisted or Execute mode)
        if mode.allows_execution() {
            let config = RuntimeConfig::new().with_mode(mode);
            let mut bridge = RuntimeBridge::new(config);
            if mode.is_execute() {
                let _ = bridge.evaluate(source_file.content());
            }
            for sym in df_analysis.active_symbol_states.keys() {
                let evidence = bridge.observe_dataframe(sym, None);
                if evidence.is_observed() {
                    runtime_evidence.push(evidence);
                }
            }
        }

        Ok(Self {
            file_path,
            source_file,
            cst: cst_tree,
            program,
            scope_model,
            cfg,
            def_use,
            reaching_defs,
            post_dominators,
            control_deps,
            df_analysis,
            pdg,
            diagnostics,
            runtime_evidence,
            mode,
        })
    }

    /// Returns true if any error diagnostics were encountered.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error)
    }
}

/// Discovers dataframe schema seeds automatically from static program usage.
pub fn discover_seeds(program: &Program, scope_model: &ScopeModel) -> Vec<DataFrameSeed> {
    let mut candidate_dfs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut assigned_symbols: BTreeSet<String> = BTreeSet::new();
    let mut aliases: BTreeMap<String, String> = BTreeMap::new();

    for stmt in &program.stmts {
        collect_stmt_seeds(
            stmt,
            &mut candidate_dfs,
            &mut assigned_symbols,
            &mut aliases,
            scope_model,
        );
    }

    // Merge any candidate columns that were recorded under an alias into its root symbol
    for (alias_name, root_sym) in &aliases {
        if let Some(cols) = candidate_dfs.remove(alias_name) {
            candidate_dfs
                .entry(root_sym.clone())
                .or_default()
                .extend(cols);
        }
    }

    let mut seeds = Vec::new();
    for (name, cols) in candidate_dfs {
        if !cols.is_empty() {
            seeds.push(DataFrameSeed::new(name, cols.into_iter().collect()));
        }
    }
    seeds
}

fn resolve_alias(name: &str, aliases: &BTreeMap<String, String>) -> String {
    let mut curr = name.to_string();
    let mut visited = BTreeSet::new();
    while let Some(parent) = aliases.get(&curr) {
        if !visited.insert(curr.clone()) {
            break;
        }
        curr = parent.clone();
    }
    curr
}

fn collect_stmt_seeds(
    stmt: &Stmt,
    candidate_dfs: &mut BTreeMap<String, BTreeSet<String>>,
    assigned_symbols: &mut BTreeSet<String>,
    aliases: &mut BTreeMap<String, String>,
    scope_model: &ScopeModel,
) {
    match stmt {
        Stmt::Assignment(assign) => match &assign.target {
            LValue::Symbol { name, .. } => {
                // Check if this is an alias assignment: target <- source_sym
                if let Expr::Identifier(src_id) = assign.value.as_ref() {
                    let root_sym = resolve_alias(&src_id.name, aliases);
                    aliases.insert(name.clone(), root_sym);
                } else {
                    aliases.remove(name);
                }

                if let Expr::Call(call) = assign.value.as_ref() {
                    if is_dataframe_constructor(call) {
                        for arg in &call.args {
                            if let Argument::Named(named) = arg {
                                candidate_dfs
                                    .entry(name.clone())
                                    .or_default()
                                    .insert(named.name.clone());
                            }
                        }
                    }
                }
                assigned_symbols.insert(name.clone());
                collect_expr_seeds(
                    &assign.value,
                    candidate_dfs,
                    assigned_symbols,
                    aliases,
                    scope_model,
                );
            }
            LValue::Extraction {
                target,
                index,
                kind: ExtractKind::Dollar,
                ..
            } => {
                if let Expr::Identifier(target_id) = target.as_ref() {
                    let df_sym = resolve_alias(&target_id.name, aliases);
                    if let Some(col) = extract_ident_name(index.as_ref()) {
                        candidate_dfs.entry(df_sym).or_default().insert(col);
                    }
                }
                collect_expr_seeds(
                    &assign.value,
                    candidate_dfs,
                    assigned_symbols,
                    aliases,
                    scope_model,
                );
            }
            _ => {
                collect_expr_seeds(
                    &assign.value,
                    candidate_dfs,
                    assigned_symbols,
                    aliases,
                    scope_model,
                );
            }
        },
        Stmt::Expr(expr_stmt) => {
            collect_expr_seeds(
                &expr_stmt.expr,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
        }
        Stmt::If(if_stmt) => {
            collect_expr_seeds(
                &if_stmt.condition,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
            for s in &if_stmt.then_branch.stmts {
                collect_stmt_seeds(s, candidate_dfs, assigned_symbols, aliases, scope_model);
            }
            if let Some(eb) = &if_stmt.else_branch {
                for s in &eb.stmts {
                    collect_stmt_seeds(s, candidate_dfs, assigned_symbols, aliases, scope_model);
                }
            }
        }
        Stmt::For(for_stmt) => {
            collect_expr_seeds(
                &for_stmt.iterable,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
            for s in &for_stmt.body.stmts {
                collect_stmt_seeds(s, candidate_dfs, assigned_symbols, aliases, scope_model);
            }
        }
        Stmt::While(while_stmt) => {
            collect_expr_seeds(
                &while_stmt.condition,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
            for s in &while_stmt.body.stmts {
                collect_stmt_seeds(s, candidate_dfs, assigned_symbols, aliases, scope_model);
            }
        }
        Stmt::Block(block) => {
            for s in &block.stmts {
                collect_stmt_seeds(s, candidate_dfs, assigned_symbols, aliases, scope_model);
            }
        }
        Stmt::Return(ret) => {
            if let Some(val) = &ret.value {
                collect_expr_seeds(val, candidate_dfs, assigned_symbols, aliases, scope_model);
            }
        }
    }
}

fn collect_expr_seeds(
    expr: &Expr,
    candidate_dfs: &mut BTreeMap<String, BTreeSet<String>>,
    assigned_symbols: &BTreeSet<String>,
    aliases: &BTreeMap<String, String>,
    scope_model: &ScopeModel,
) {
    match expr {
        Expr::Extract(extract) if extract.kind == ExtractKind::Dollar => {
            if let Expr::Identifier(target_id) = extract.target.as_ref() {
                let df_sym = resolve_alias(&target_id.name, aliases);
                if let Some(col) = extract_ident_name(&extract.index) {
                    candidate_dfs.entry(df_sym).or_default().insert(col);
                }
            }
        }
        Expr::Pipe(pipe) => {
            if let Some(root_df) = find_pipe_root_df(&pipe.left) {
                let resolved_root = resolve_alias(&root_df, aliases);
                collect_pipe_verb_cols(
                    &pipe.right,
                    &resolved_root,
                    candidate_dfs,
                    assigned_symbols,
                    scope_model,
                );
            }
            collect_expr_seeds(
                &pipe.left,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
            collect_expr_seeds(
                &pipe.right,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
        }
        Expr::Call(call) => {
            if let Expr::Identifier(fn_id) = call.callee.as_ref() {
                if is_dataframe_verb(&fn_id.name) && !call.args.is_empty() {
                    if let Expr::Identifier(first_arg) = call.args[0].value() {
                        let df_sym = resolve_alias(&first_arg.name, aliases);
                        for arg in &call.args[1..] {
                            collect_verb_arg_cols(
                                arg,
                                &df_sym,
                                candidate_dfs,
                                assigned_symbols,
                                scope_model,
                            );
                        }
                    }
                }
            }
            for arg in &call.args {
                collect_expr_seeds(
                    arg.value(),
                    candidate_dfs,
                    assigned_symbols,
                    aliases,
                    scope_model,
                );
            }
        }
        Expr::BinaryOp(b) => {
            collect_expr_seeds(
                &b.left,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
            collect_expr_seeds(
                &b.right,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
        }
        Expr::UnaryOp(u) => {
            collect_expr_seeds(
                &u.operand,
                candidate_dfs,
                assigned_symbols,
                aliases,
                scope_model,
            );
        }
        _ => {}
    }
}

fn find_pipe_root_df(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Identifier(id) => Some(id.name.clone()),
        Expr::Pipe(pipe) => find_pipe_root_df(&pipe.left),
        Expr::Call(call) => {
            if !call.args.is_empty() {
                find_pipe_root_df(call.args[0].value())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn collect_pipe_verb_cols(
    verb_expr: &Expr,
    df_sym: &str,
    candidate_dfs: &mut BTreeMap<String, BTreeSet<String>>,
    assigned_symbols: &BTreeSet<String>,
    scope_model: &ScopeModel,
) {
    if let Expr::Call(call) = verb_expr {
        if let Expr::Identifier(fn_id) = call.callee.as_ref() {
            if is_dataframe_verb(&fn_id.name) {
                for arg in &call.args {
                    collect_verb_arg_cols(
                        arg,
                        df_sym,
                        candidate_dfs,
                        assigned_symbols,
                        scope_model,
                    );
                }
            }
        }
    }
}

fn collect_verb_arg_cols(
    arg: &Argument,
    df_sym: &str,
    candidate_dfs: &mut BTreeMap<String, BTreeSet<String>>,
    assigned_symbols: &BTreeSet<String>,
    scope_model: &ScopeModel,
) {
    match arg {
        Argument::Named(named) => {
            collect_free_cols(
                &named.value,
                df_sym,
                candidate_dfs,
                assigned_symbols,
                scope_model,
            );
        }
        Argument::Positional(pos) => {
            if let Expr::Identifier(id) = pos.value.as_ref() {
                candidate_dfs
                    .entry(df_sym.to_string())
                    .or_default()
                    .insert(id.name.clone());
            } else {
                collect_free_cols(
                    &pos.value,
                    df_sym,
                    candidate_dfs,
                    assigned_symbols,
                    scope_model,
                );
            }
        }
    }
}

fn collect_free_cols(
    expr: &Expr,
    df_sym: &str,
    candidate_dfs: &mut BTreeMap<String, BTreeSet<String>>,
    assigned_symbols: &BTreeSet<String>,
    scope_model: &ScopeModel,
) {
    match expr {
        Expr::Identifier(id) => {
            let is_global_var = scope_model
                .resolve_symbol(ScopeId(0), &id.name)
                .map(|sym_id| !scope_model.symbol(sym_id).defs.is_empty())
                .unwrap_or(false);
            if !is_global_var && !assigned_symbols.contains(&id.name) {
                candidate_dfs
                    .entry(df_sym.to_string())
                    .or_default()
                    .insert(id.name.clone());
            }
        }
        Expr::BinaryOp(b) => {
            collect_free_cols(
                &b.left,
                df_sym,
                candidate_dfs,
                assigned_symbols,
                scope_model,
            );
            collect_free_cols(
                &b.right,
                df_sym,
                candidate_dfs,
                assigned_symbols,
                scope_model,
            );
        }
        Expr::UnaryOp(u) => {
            collect_free_cols(
                &u.operand,
                df_sym,
                candidate_dfs,
                assigned_symbols,
                scope_model,
            );
        }
        Expr::Call(call) => {
            for a in &call.args {
                collect_free_cols(
                    a.value(),
                    df_sym,
                    candidate_dfs,
                    assigned_symbols,
                    scope_model,
                );
            }
        }
        _ => {}
    }
}

fn is_dataframe_verb(name: &str) -> bool {
    matches!(
        name,
        "mutate" | "transmute" | "filter" | "select" | "rename" | "transform"
    )
}

fn is_dataframe_constructor(call: &rparse_ir::CallExpr) -> bool {
    if let Expr::Identifier(id) = call.callee.as_ref() {
        matches!(
            id.name.as_str(),
            "data.frame" | "tibble" | "read.csv" | "read_csv"
        )
    } else {
        false
    }
}

fn extract_ident_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Identifier(id) => Some(id.name.clone()),
        Expr::Literal(lit) => match &lit.kind {
            rparse_ir::LiteralKind::String(s) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}
