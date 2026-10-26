//! Comprehensive integration, command, golden, runtime, and benchmark test suite for R-PARSE CLI.

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::normalize_cst;
use rparse_parser::RParser;
use rparse_pdg::build_pdg;
use rparse_runtime::json::JsonValue;
use rparse_scope::resolve_scopes;
use rparse_slicing::{compute_backward_slice, SliceCriterion};

use crate::exit_code::*;
use crate::{run_cli, run_cli_with_reader};

fn create_temp_script(prefix: &str, content: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let path =
        std::env::temp_dir().join(format!("rparse_{}_{}_{}.R", prefix, std::process::id(), id));
    fs::write(&path, content).expect("write temp script");
    path
}

// ============================================================================
// 1. CLI ARGUMENT & USAGE TESTS
// ============================================================================

mod args_tests {
    use super::*;

    #[test]
    fn test_cli_help() {
        let (code, out) = run_cli(vec!["rparse".to_string(), "--help".to_string()]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE"));
        assert!(out.contains("analyze"));
        assert!(out.contains("provenance"));
        assert!(out.contains("slice"));
    }

    #[test]
    fn test_cli_subcommand_help() {
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            "--help".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("rparse analyze"));

        let (code2, out2) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            "-h".to_string(),
        ]);
        assert_eq!(code2, EXIT_SUCCESS);
        assert!(out2.contains("rparse provenance"));

        let (code3, out3) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            "--help".to_string(),
        ]);
        assert_eq!(code3, EXIT_SUCCESS);
        assert!(out3.contains("rparse slice"));
    }

    #[test]
    fn test_cli_version() {
        let (code, out) = run_cli(vec!["rparse".to_string(), "--version".to_string()]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.starts_with("rparse "));
    }

    #[test]
    fn test_cli_unknown_command() {
        let (code, out) = run_cli(vec!["rparse".to_string(), "unknown_cmd".to_string()]);
        assert_eq!(code, EXIT_INVALID_USAGE);
        assert!(out.contains("Unknown command 'unknown_cmd'"));
    }

    #[test]
    fn test_cli_unknown_flag() {
        let (code, out) = run_cli(vec!["rparse".to_string(), "--invalid-flag".to_string()]);
        assert_eq!(code, EXIT_INVALID_USAGE);
        assert!(out.contains("Unknown flag '--invalid-flag'"));
    }

    #[test]
    fn test_cli_missing_file_arg() {
        let (code, out) = run_cli(vec!["rparse".to_string(), "analyze".to_string()]);
        assert_eq!(code, EXIT_INVALID_USAGE);
        assert!(out.contains("Missing required <FILE>"));
    }

    #[test]
    fn test_cli_nonexistent_file() {
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            "nonexistent_script_12345.R".to_string(),
        ]);
        assert_eq!(code, EXIT_INPUT_ERROR);
        assert!(out.contains("File does not exist"));
    }

    #[test]
    fn test_cli_directory_input_rejected() {
        let temp_dir = std::env::temp_dir();
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            temp_dir.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_INPUT_ERROR);
        assert!(out.contains("Path is a directory"));
    }

    #[test]
    fn test_cli_missing_column_flag() {
        let fixture_path = create_temp_script("args", "x <- 1\n");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            fixture_path.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_INVALID_USAGE);
        assert!(out.contains("Missing required '--column"));
        let _ = fs::remove_file(fixture_path);
    }

    #[test]
    fn test_cli_missing_criterion_flag() {
        let fixture_path = create_temp_script("args", "x <- 1\n");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            fixture_path.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_INVALID_USAGE);
        assert!(out.contains("Missing required '--criterion"));
        let _ = fs::remove_file(fixture_path);
    }
}

// ============================================================================
// 2. CORE CLI COMMANDS (ANALYZE, PROVENANCE, SLICE)
// ============================================================================

mod commands_tests {
    use super::*;

    #[test]
    fn test_analyze_text_format() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE STATIC ANALYSIS"));
        assert!(out.contains("Status:       SUCCESS"));
        assert!(out.contains("Program Dependence Graph:"));
        assert!(out.contains("Nodes:"));
        assert!(out.contains("Edges:"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_analyze_json_format() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("tool").and_then(|v| v.as_str()), Some("rparse"));
        assert_eq!(
            parsed.get("command").and_then(|v| v.as_str()),
            Some("analyze")
        );
        assert_eq!(
            parsed.get("status").and_then(|v| v.as_str()),
            Some("success")
        );
        assert!(parsed.get("summary").is_some());
        assert!(parsed.get("dataframes").is_some());
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_analyze_empty_file() {
        let file = create_temp_script("cmd", "");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("Status:       SUCCESS"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_analyze_utf8_source() {
        let script = r#"
            # Unicode comments: ☕ λ ∑
            df2 <- df %>% mutate(prix_total = prix * quantité)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("Status:       SUCCESS"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_analyze_syntax_error_file() {
        let script = r#"
            df2 <- df %>% mutate(
            # unclosed syntax error
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
        ]);
        assert_eq!(code, EXIT_ANALYSIS_FAILURE);
        assert!(out.contains("ERROR"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_provenance_command_text() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            file.to_string_lossy().to_string(),
            "--column".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE COLUMN PROVENANCE"));
        assert!(out.contains("Target Column:     df2$total"));
        assert!(out.contains("df$price"));
        assert!(out.contains("df$quantity"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_provenance_command_json() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            file.to_string_lossy().to_string(),
            "--column".to_string(),
            "df2$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("tool").and_then(|v| v.as_str()), Some("rparse"));
        assert_eq!(
            parsed.get("command").and_then(|v| v.as_str()),
            Some("provenance")
        );
        assert_eq!(
            parsed.get("status").and_then(|v| v.as_str()),
            Some("success")
        );

        let target = parsed.get("target").expect("target field");
        assert_eq!(target.get("column").and_then(|v| v.as_str()), Some("total"));
        assert_eq!(
            target.get("dataframe").and_then(|v| v.as_str()),
            Some("df2")
        );

        let direct = parsed
            .get("direct_inputs")
            .and_then(|v| v.as_array())
            .expect("direct inputs array");
        assert_eq!(direct.len(), 2);
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_provenance_nonexistent_column() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            file.to_string_lossy().to_string(),
            "--column".to_string(),
            "df2$nonexistent".to_string(),
        ]);
        assert_eq!(code, EXIT_ANALYSIS_FAILURE);
        assert!(out.contains("Column 'nonexistent' not found in dataframe 'df2'"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_slice_command_text() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity, unused = age * 2)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            file.to_string_lossy().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE BACKWARD STATIC SLICE"));
        assert!(out.contains("Criterion:         df2$total"));
        assert!(out.contains("Confidence:        COMPLETE"));
        assert!(out.contains("Contains Unknowns: NO"));
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_slice_command_json() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity, unused = age * 2)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            file.to_string_lossy().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("tool").and_then(|v| v.as_str()), Some("rparse"));
        assert_eq!(
            parsed.get("command").and_then(|v| v.as_str()),
            Some("slice")
        );
        assert_eq!(
            parsed.get("status").and_then(|v| v.as_str()),
            Some("success")
        );
        assert_eq!(
            parsed.get("confidence").and_then(|v| v.as_str()),
            Some("COMPLETE")
        );
        assert_eq!(
            parsed.get("contains_unknowns").and_then(|v| v.as_bool()),
            Some(false)
        );
        assert!(parsed.get("statements").is_some());
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_slice_invalid_criterion() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("cmd", script);
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            file.to_string_lossy().to_string(),
            "--criterion".to_string(),
            "unknown_df$total".to_string(),
        ]);
        assert_eq!(code, EXIT_ANALYSIS_FAILURE);
        assert!(out.contains("Error:"));
        let _ = fs::remove_file(file);
    }
}

// ============================================================================
// 3. GOLDEN BENCHMARKS & DETERMINISM
// ============================================================================

mod golden_tests {
    use super::*;

    #[test]
    fn test_golden_benchmark_1_direct_derivation() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("golden", script);

        let (code, prov_out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            file.to_string_lossy().to_string(),
            "--column".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(prov_out.contains("Target Column:     df2$total"));
        assert!(prov_out.contains("← df$price"));
        assert!(prov_out.contains("← df$quantity"));
        assert!(prov_out.contains("df$price, df$quantity → df2$total"));

        let (code, slice_out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            file.to_string_lossy().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(slice_out.contains("Confidence:        COMPLETE"));
        assert!(slice_out.contains("Contains Unknowns: NO"));
        assert!(slice_out.contains("[PARTIAL]"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_golden_benchmark_2_multi_step_provenance() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
            df3 <- df2 %>% mutate(tax = total * 0.18)
        "#;
        let file = create_temp_script("golden", script);

        let (code, prov_out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            file.to_string_lossy().to_string(),
            "--column".to_string(),
            "df3$tax".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(prov_out.contains("Target Column:     df3$tax"));
        assert!(prov_out.contains("Direct Input Columns:"));
        assert!(prov_out.contains("← df2$total"));
        assert!(prov_out.contains("Transitive Source Columns:"));
        assert!(prov_out.contains("← df$price"));
        assert!(prov_out.contains("← df$quantity"));
        assert!(prov_out.contains("df$price, df$quantity → df2$total"));
        assert!(prov_out.contains("df2$total → df3$tax"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_golden_benchmark_7_partial_statement_retention() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity, unused = age * 2)
        "#;
        let file = create_temp_script("golden", script);

        let (code, slice_out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            file.to_string_lossy().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(slice_out.contains("Partially Retained Statements:1"));
        assert!(slice_out.contains("[PARTIAL]"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_golden_rename_and_select() {
        let script = r#"
            df2 <- df %>% rename(cost = price)
            df3 <- df2 %>% select(cost, quantity)
        "#;
        let file = create_temp_script("golden", script);

        let (code, prov_out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            file.to_string_lossy().to_string(),
            "--column".to_string(),
            "df3$cost".to_string(),
            "--seed".to_string(),
            "df=price,quantity".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(prov_out.contains("Target Column:     df3$cost"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_golden_determinism_repeated_execution() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity, unused = age * 2)
            df3 <- df2 %>% mutate(tax = total * 0.18)
        "#;
        let file = create_temp_script("golden", script);

        let mut prev_prov_text = String::new();
        let mut prev_slice_json = String::new();

        for i in 0..5 {
            let (code, prov_out) = run_cli(vec![
                "rparse".to_string(),
                "provenance".to_string(),
                file.to_string_lossy().to_string(),
                "--column".to_string(),
                "df3$tax".to_string(),
            ]);
            assert_eq!(code, EXIT_SUCCESS);

            let (code2, slice_out) = run_cli(vec![
                "rparse".to_string(),
                "slice".to_string(),
                file.to_string_lossy().to_string(),
                "--criterion".to_string(),
                "df3$tax".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ]);
            assert_eq!(code2, EXIT_SUCCESS);

            if i == 0 {
                prev_prov_text = prov_out;
                prev_slice_json = slice_out;
            } else {
                assert_eq!(
                    prov_out, prev_prov_text,
                    "Provenance text must be byte-for-byte identical across runs"
                );
                assert_eq!(
                    slice_out, prev_slice_json,
                    "Slice JSON must be byte-for-byte identical across runs"
                );
            }
        }

        let _ = fs::remove_file(file);
    }
}

// ============================================================================
// 4. RUNTIME MODES & ISOLATION
// ============================================================================

mod runtime_tests {
    use super::*;

    #[test]
    fn test_runtime_mode_default_is_static() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("rt", script);

        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("mode").and_then(|v| v.as_str()), Some("STATIC"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_runtime_mode_static_never_executes_r() {
        let script = r#"
            # Code that would fail or crash if evaluated in R:
            stop("This must never be executed by GNU R in static mode")
        "#;
        let file = create_temp_script("rt", script);

        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
            "--mode".to_string(),
            "static".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("mode").and_then(|v| v.as_str()), Some("STATIC"));

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_runtime_mode_assisted_flag() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("rt", script);

        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
            "--mode".to_string(),
            "assisted".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(
            parsed.get("mode").and_then(|v| v.as_str()),
            Some("ASSISTED")
        );

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_runtime_mode_execute_flag() {
        let script = r#"
            df2 <- df %>% mutate(total = price * quantity)
        "#;
        let file = create_temp_script("rt", script);

        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            file.to_string_lossy().to_string(),
            "--mode".to_string(),
            "execute".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("mode").and_then(|v| v.as_str()), Some("EXECUTE"));

        let _ = fs::remove_file(file);
    }
}

// ============================================================================
// 5. 8-DIMENSION BENCHMARK SUITE
// ============================================================================

mod benchmark_suite {
    use super::*;

    fn get_benchmark_script() -> &'static str {
        r#"
            raw_data <- df %>%
                filter(price > 0, quantity > 0) %>%
                mutate(
                    subtotal = price * quantity,
                    discount = subtotal * 0.05
                )

            processed <- raw_data %>%
                mutate(
                    total_before_tax = subtotal - discount,
                    tax = total_before_tax * 0.18,
                    final_total = total_before_tax + tax
                ) %>%
                select(final_total, tax, price)

            summary_df <- processed %>%
                mutate(
                    effective_rate = tax / final_total
                )
        "#
    }

    #[test]
    fn test_benchmarks_8_dimensions() {
        let source = get_benchmark_script();
        let source_bytes = source.len();

        println!("\n========================================================");
        println!(" R-PARSE BENCHMARK SUITE — 8 ANALYSIS DIMENSIONS");
        println!(" Input size: {} bytes", source_bytes);
        println!("========================================================");

        // 1. Parser
        let t0 = Instant::now();
        let mut parser = RParser::new().expect("create parser");
        let cst = parser.parse(source).expect("parse source");
        let t_parse = t0.elapsed();
        println!(" [1/8] Parser Performance:           {:>8.3?}", t_parse);

        // 2. IR Construction
        let t0 = Instant::now();
        let program = normalize_cst(&cst);
        let t_ir = t0.elapsed();
        let stmt_count = program.stmts.len();
        println!(
            " [2/8] IR Normalization:             {:>8.3?} ({} statements)",
            t_ir, stmt_count
        );

        // 3. Scope Analysis
        let t0 = Instant::now();
        let scope_model = resolve_scopes(&program);
        let t_scope = t0.elapsed();
        println!(" [3/8] Scope Resolution:             {:>8.3?}", t_scope);

        // 4. CFG & Dataflow
        let t0 = Instant::now();
        let cfg = build_cfg(&program).expect("build cfg");
        let def_use = collect_def_use(&program, &scope_model, &cfg);
        let reaching_defs = compute_reaching_definitions(&cfg, &def_use);
        let post_dom = compute_post_dominators(&cfg);
        let control_deps = compute_control_dependencies(&cfg, &post_dom);
        let t_dataflow = t0.elapsed();
        println!(" [4/8] CFG & Dataflow Analysis:      {:>8.3?}", t_dataflow);

        // 5. DataFrame & Provenance
        let t0 = Instant::now();
        let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
        let df_analysis = analyze_dataframes(&program, &scope_model, &seeds);
        let t_df = t0.elapsed();
        let prov_edges = df_analysis.provenance().edges().len();
        println!(
            " [5/8] DataFrame & Provenance:       {:>8.3?} ({} prov edges)",
            t_df, prov_edges
        );

        // 6. Unified PDG Construction
        let t0 = Instant::now();
        let pdg = build_pdg(
            &program,
            &scope_model,
            &cfg,
            &def_use,
            &reaching_defs,
            &control_deps,
            &df_analysis,
        );
        let t_pdg = t0.elapsed();
        let node_count = pdg.nodes().len();
        let edge_count = pdg.edges().len();
        println!(
            " [6/8] Unified PDG Construction:     {:>8.3?} ({} nodes, {} edges)",
            t_pdg, node_count, edge_count
        );

        // 7. Column-Aware Slicing
        let t0 = Instant::now();
        let criterion = SliceCriterion::ColumnByName {
            dataframe_symbol: "summary_df".to_string(),
            column_name: "effective_rate".to_string(),
        };
        let slice_res = compute_backward_slice(&criterion, &pdg, &df_analysis, &scope_model)
            .expect("compute backward slice");
        let t_slice = t0.elapsed();
        println!(
            " [7/8] Backward Slicing:             {:>8.3?} ({} retained elements)",
            t_slice,
            slice_res.retained_elements.len()
        );

        // 8. End-to-End Pipeline
        let t0 = Instant::now();
        let temp_file =
            std::env::temp_dir().join(format!("rparse_bench_e2e_{}.R", std::process::id()));
        std::fs::write(&temp_file, source).expect("write bench file");
        let (code, _) = crate::run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            temp_file.to_string_lossy().to_string(),
            "--criterion".to_string(),
            "summary_df$effective_rate".to_string(),
        ]);
        let t_e2e = t0.elapsed();
        let _ = std::fs::remove_file(temp_file);
        assert_eq!(code, 0);
        println!(" [8/8] End-to-End CLI Pipeline:      {:>8.3?}", t_e2e);

        println!("========================================================\n");

        assert!(node_count > 0);
        assert!(edge_count > 0);
        assert!(stmt_count > 0);
    }
}

// ============================================================================
// STDIN INPUT TESTS ("-")
// ============================================================================

mod stdin_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_analyze_from_stdin_text() {
        let script = "x <- 10\ny <- 20\nz <- x + y\n";
        let (code, out) = run_cli_with_reader(
            vec!["rparse".to_string(), "analyze".to_string(), "-".to_string()],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE STATIC ANALYSIS"));
        assert!(out.contains("File:         -"));
        assert!(out.contains("Status:       SUCCESS"));
        assert!(out.contains("Statements:    3"));
    }

    #[test]
    fn test_analyze_from_stdin_json() {
        let script = "x <- 10\ny <- 20\nz <- x + y\n";
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(parsed.get("tool").and_then(|v| v.as_str()), Some("rparse"));
        assert_eq!(
            parsed.get("command").and_then(|v| v.as_str()),
            Some("analyze")
        );
        assert_eq!(parsed.get("file").and_then(|v| v.as_str()), Some("-"));
        assert_eq!(
            parsed.get("status").and_then(|v| v.as_str()),
            Some("success")
        );
    }

    #[test]
    fn test_provenance_from_stdin() {
        let script =
            "df <- data.frame(price = 10, quantity = 5)\ndf$total <- df$price * df$quantity\n";
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "provenance".to_string(),
                "-".to_string(),
                "--column".to_string(),
                "df$total".to_string(),
            ],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE COLUMN PROVENANCE"));
        assert!(out.contains("Target Column:     df$total"));
        assert!(out.contains("df$price, df$quantity → df$total"));
    }

    #[test]
    fn test_provenance_from_stdin_json() {
        let script =
            "df <- data.frame(price = 10, quantity = 5)\ndf$total <- df$price * df$quantity\n";
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "provenance".to_string(),
                "-".to_string(),
                "--column".to_string(),
                "df$total".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(
            parsed.get("command").and_then(|v| v.as_str()),
            Some("provenance")
        );
        assert_eq!(parsed.get("file").and_then(|v| v.as_str()), Some("-"));
        assert_eq!(
            parsed.get("status").and_then(|v| v.as_str()),
            Some("success")
        );
    }

    #[test]
    fn test_slice_from_stdin() {
        let script = "df <- data.frame(price = 10, quantity = 5)\ndf$total <- df$price * df$quantity\ndf2 <- mutate(df, tax = total * 0.1)\n";
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "slice".to_string(),
                "-".to_string(),
                "--criterion".to_string(),
                "df$total".to_string(),
            ],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("R-PARSE BACKWARD STATIC SLICE"));
        assert!(out.contains("Criterion:         df$total"));
        assert!(out.contains("[FULL] Line 1:   df <- data.frame(price = 10, quantity = 5)"));
        assert!(out.contains("[FULL] Line 2:   df$total <- df$price * df$quantity"));
        assert!(out.contains("[NONE] Line 3:   df2 <- mutate(df, tax = total * 0.1)"));
    }

    #[test]
    fn test_slice_from_stdin_json() {
        let script = "df <- data.frame(price = 10, quantity = 5)\ndf$total <- df$price * df$quantity\ndf2 <- mutate(df, tax = total * 0.1)\n";
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "slice".to_string(),
                "-".to_string(),
                "--criterion".to_string(),
                "df$total".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid json output");
        assert_eq!(
            parsed.get("command").and_then(|v| v.as_str()),
            Some("slice")
        );
        assert_eq!(parsed.get("file").and_then(|v| v.as_str()), Some("-"));
        assert_eq!(
            parsed.get("status").and_then(|v| v.as_str()),
            Some("success")
        );
    }

    #[test]
    fn test_stdin_with_flags_before_dash() {
        let script =
            "df <- data.frame(price = 10, quantity = 5)\ndf$total <- df$price * df$quantity\n";
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "provenance".to_string(),
                "--column".to_string(),
                "df$total".to_string(),
                "-".to_string(),
            ],
            Cursor::new(script),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("Target Column:     df$total"));
    }
}

// ============================================================================
// 12. GRAPH EXPORT & TOPOLOGY SERIALIZATION TESTS
// ============================================================================

mod graph_export_tests {
    use super::*;
    use std::io::Cursor;

    const CANONICAL_SCRIPT: &str = r#"
        df <- data.frame(price = 10, quantity = 5)
        df$total <- df$price * df$quantity
    "#;

    #[test]
    fn test_cli_analyze_graph_pdg_dot() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "pdg".to_string(),
                "--format".to_string(),
                "dot".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("digraph PDG {"));
        assert!(out.contains("node [fontname=\"Courier\"];"));
        assert!(out.contains("ColumnPort"));
        assert!(out.contains("Statement"));
        assert!(out.contains(" -> "));
    }

    #[test]
    fn test_cli_analyze_graph_pdg_default_format_is_dot() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "pdg".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.starts_with("digraph PDG {"));
    }

    #[test]
    fn test_cli_analyze_graph_pdg_json() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "pdg".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid pdg json output");
        assert_eq!(parsed.get("graph_type").unwrap().as_str(), Some("pdg"));

        let nodes = parsed.get("nodes").unwrap().as_array().unwrap();
        assert!(!nodes.is_empty());
        let edges = parsed.get("edges").unwrap().as_array().unwrap();
        assert!(!edges.is_empty());

        let summary = parsed.get("summary").unwrap();
        assert!(summary.get("node_count").unwrap().as_u64().unwrap() > 0);
        assert!(summary.get("edge_count").unwrap().as_u64().unwrap() > 0);
        assert!(summary.get("column_port_count").unwrap().as_u64().unwrap() >= 2);
    }

    #[test]
    fn test_cli_analyze_graph_provenance_dot() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "provenance".to_string(),
                "--format".to_string(),
                "dot".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("digraph ColumnProvenance {"));
        assert!(out.contains("BaseRDirectAssignment"));
        assert!(out.contains(" -> "));
    }

    #[test]
    fn test_cli_analyze_graph_provenance_json() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "provenance".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid provenance json output");
        assert_eq!(
            parsed.get("graph_type").unwrap().as_str(),
            Some("provenance")
        );

        let cols = parsed.get("columns").unwrap().as_array().unwrap();
        assert!(!cols.is_empty());
        let derivs = parsed.get("derivations").unwrap().as_array().unwrap();
        assert!(!derivs.is_empty());
    }

    #[test]
    fn test_cli_analyze_graph_cfg_dot() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "cfg".to_string(),
                "--format".to_string(),
                "dot".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("digraph CFG {"));
    }

    #[test]
    fn test_cli_analyze_graph_cfg_json() {
        let (code, out) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--graph".to_string(),
                "cfg".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid cfg json output");
        assert_eq!(parsed.get("graph_type").unwrap().as_str(), Some("cfg"));
        let nodes = parsed.get("nodes").unwrap().as_array().unwrap();
        assert!(!nodes.is_empty());
    }

    #[test]
    fn test_cli_invalid_combinations() {
        // --format dot without --graph
        let (code1, out1) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            "-".to_string(),
            "--format".to_string(),
            "dot".to_string(),
        ]);
        assert_ne!(code1, EXIT_SUCCESS);
        assert!(out1.contains("'--format dot' requires '--graph"));

        // --graph with --format text
        let (code2, out2) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            "-".to_string(),
            "--graph".to_string(),
            "pdg".to_string(),
            "--format".to_string(),
            "text".to_string(),
        ]);
        assert_ne!(code2, EXIT_SUCCESS);
        assert!(out2.contains("does not support 'text'"));

        // Invalid graph type
        let (code3, out3) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            "-".to_string(),
            "--graph".to_string(),
            "invalid_graph".to_string(),
        ]);
        assert_ne!(code3, EXIT_SUCCESS);
        assert!(out3.contains("Invalid graph type 'invalid_graph'"));

        // provenance command with --format dot
        let (code4, out4) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            "-".to_string(),
            "--column".to_string(),
            "df$total".to_string(),
            "--format".to_string(),
            "dot".to_string(),
        ]);
        assert_ne!(code4, EXIT_SUCCESS);
        assert!(out4.contains("only supported with 'analyze"));

        // slice command with --format dot
        let (code5, out5) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            "-".to_string(),
            "--criterion".to_string(),
            "df$total".to_string(),
            "--format".to_string(),
            "dot".to_string(),
        ]);
        assert_ne!(code5, EXIT_SUCCESS);
        assert!(out5.contains("only supported with 'analyze"));
    }

    #[test]
    fn test_cli_backward_compatibility() {
        // Default analyze text output
        let (code_text, out_text) = run_cli_with_reader(
            vec!["rparse".to_string(), "analyze".to_string(), "-".to_string()],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code_text, EXIT_SUCCESS);
        assert!(out_text.contains("R-PARSE STATIC ANALYSIS"));

        // Standard analyze JSON summary
        let (code_json, out_json) = run_cli_with_reader(
            vec![
                "rparse".to_string(),
                "analyze".to_string(),
                "-".to_string(),
                "--format".to_string(),
                "json".to_string(),
            ],
            Cursor::new(CANONICAL_SCRIPT),
        );
        assert_eq!(code_json, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out_json).expect("valid analyze json");
        assert_eq!(parsed.get("command").unwrap().as_str(), Some("analyze"));
        assert_eq!(parsed.get("tool").unwrap().as_str(), Some("rparse"));
        assert!(parsed.get("summary").is_some());
    }
}

// ============================================================================
// 11. PHASE C MILESTONE 2: SHOWCASE FIXTURE CORPUS & SUB-STATEMENT ATTRIBUTION
// ============================================================================

mod milestone2_tests {
    use super::*;

    fn find_fixture(name: &str) -> PathBuf {
        let mut dir = std::env::current_dir().unwrap();
        loop {
            let candidate = dir.join("fixtures").join(name);
            if candidate.exists() {
                return candidate;
            }
            let candidate2 = dir.join("implementation").join("fixtures").join(name);
            if candidate2.exists() {
                return candidate2;
            }
            if !dir.pop() {
                panic!("Could not find fixture {}", name);
            }
        }
    }

    #[test]
    fn test_all_showcase_fixtures_analyze() {
        let fixtures = [
            "showcase_pipeline.R",
            "partial_retention.R",
            "filter_provenance.R",
            "uncertainty.R",
        ];
        for fix in &fixtures {
            let path = find_fixture(fix);
            let (code, out) = run_cli(vec![
                "rparse".to_string(),
                "analyze".to_string(),
                path.to_str().unwrap().to_string(),
            ]);
            assert_eq!(
                code, EXIT_SUCCESS,
                "Fixture {} failed to analyze: {}",
                fix, out
            );
            assert!(out.contains("R-PARSE STATIC ANALYSIS"));
        }
    }

    #[test]
    fn test_partial_retention_text_attribution() {
        let path = find_fixture("partial_retention.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS, "Slice failed: {}", out);
        assert!(out.contains("Confidence:        COMPLETE"));
        assert!(out.contains("Contains Unknowns: NO"));
        assert!(out.contains("[PARTIAL] Line 20:"));
        assert!(out.contains("[KEEP] price = 25"));
        assert!(out.contains("[KEEP] quantity = 4"));
        assert!(out.contains("[DROP] age = 40"));
        assert!(out.contains("[PARTIAL] Line 21:"));
        assert!(out.contains("[KEEP] total = price * quantity"));
        assert!(out.contains("[DROP] unused = age * 2"));
    }

    #[test]
    fn test_partial_retention_json_attribution() {
        let path = find_fixture("partial_retention.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS, "Slice JSON failed: {}", out);
        let parsed = JsonValue::parse(&out).expect("valid slice json");
        assert_eq!(parsed.get("command").unwrap().as_str(), Some("slice"));
        assert_eq!(parsed.get("confidence").unwrap().as_str(), Some("COMPLETE"));
        assert_eq!(
            parsed.get("contains_unknowns").unwrap().as_bool(),
            Some(false)
        );
        assert_eq!(parsed.get("criterion").unwrap().as_str(), Some("df2$total"));

        let stmts = parsed.get("statements").unwrap().as_array().unwrap();
        assert!(!stmts.is_empty());

        let mutate_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(21))
            .expect("mutate statement at line 21");

        assert_eq!(
            mutate_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let span = mutate_stmt.get("span").unwrap();
        assert!(span.get("start_line").is_some());
        assert!(span.get("end_line").is_some());

        let sub_elements = mutate_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(sub_elements.len(), 2);

        let kept = &sub_elements[0];
        assert_eq!(
            kept.get("label").unwrap().as_str(),
            Some("total = price * quantity")
        );
        assert_eq!(kept.get("state").unwrap().as_str(), Some("retained"));
        let kept_span = kept.get("span").unwrap();
        assert_eq!(kept_span.get("start_line").unwrap().as_u64(), Some(21));

        let dropped = &sub_elements[1];
        assert_eq!(
            dropped.get("label").unwrap().as_str(),
            Some("unused = age * 2")
        );
        assert_eq!(dropped.get("state").unwrap().as_str(), Some("excluded"));
        let dropped_span = dropped.get("span").unwrap();
        assert_eq!(dropped_span.get("start_line").unwrap().as_u64(), Some(21));
    }

    #[test]
    fn test_showcase_pipeline_slicing() {
        let path = find_fixture("showcase_pipeline.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df2$total".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS, "Slice failed: {}", out);
        // Line 25: data.frame
        assert!(out.contains("[KEEP] price = 100"));
        assert!(out.contains("[KEEP] quantity = 2"));
        assert!(out.contains("[DROP] age = 30"));
        // Line 26: filter
        assert!(out.contains("[FULL] Line 26:  filtered <- filter(df, price > 0)"));
        // Line 27: mutate
        assert!(out.contains("[KEEP] total = price * quantity"));
        assert!(out.contains("[DROP] discount = age * 0.05"));
        // Line 28: downstream consumer excluded
        assert!(out.contains("[NONE] Line 28:  df3 <- mutate(df2, tax = total * 0.18)"));
    }

    #[test]
    fn test_filter_provenance_row_selection_dependency() {
        let path = find_fixture("filter_provenance.R");
        // 1. Slicing preserves row filter statements
        let (code_slice, out_slice) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df3$total".to_string(),
        ]);
        assert_eq!(code_slice, EXIT_SUCCESS);
        assert!(out_slice.contains("[FULL] Line 22:  df1 <- filter(df, price > 0)"));
        assert!(out_slice.contains("[FULL] Line 23:  df2 <- filter(df1, status == 1)"));
        assert!(out_slice.contains("[KEEP] total = price * quantity"));

        // 2. PDG export contains RowSelectionDependency
        let (code_pdg, out_pdg) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--graph".to_string(),
            "pdg".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_pdg, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out_pdg).expect("valid pdg json");
        let edges = parsed.get("edges").unwrap().as_array().unwrap();
        let row_sel_edges: Vec<_> = edges
            .iter()
            .filter(|e| e.get("kind").and_then(|k| k.as_str()) == Some("RowSelectionDependency"))
            .collect();
        assert!(
            !row_sel_edges.is_empty(),
            "PDG must contain RowSelectionDependency edges"
        );

        // CFG ControlDependency should not be falsely asserted on sequential statements
        let control_dep_edges: Vec<_> = edges
            .iter()
            .filter(|e| e.get("kind").and_then(|k| k.as_str()) == Some("ControlDependency"))
            .collect();
        assert!(
            control_dep_edges.is_empty(),
            "Sequential filters must not have CFG ControlDependency edges"
        );
    }

    #[test]
    fn test_uncertainty_explicit_unknown() {
        let path = find_fixture("uncertainty.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df2$dynamic_col".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("Confidence:        PARTIAL_CONTAINS_UNKNOWNS"));
        assert!(out.contains("Contains Unknowns: YES"));
        // Does NOT falsely attribute to price or quantity
        assert!(out.contains("[DROP] price = 10"));
        assert!(out.contains("[DROP] quantity = 5"));
        assert!(out.contains("[KEEP] dynamic_col = .data[[get_col_name()]]"));

        // Verify JSON contains_unknowns flag
        let (code_json, out_json) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df2$dynamic_col".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_json, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out_json).expect("valid uncertainty json");
        assert_eq!(
            parsed.get("contains_unknowns").unwrap().as_bool(),
            Some(true)
        );
        assert_eq!(
            parsed.get("confidence").unwrap().as_str(),
            Some("PARTIAL_CONTAINS_UNKNOWNS")
        );
    }

    #[test]
    fn test_milestone2_slice_determinism() {
        let fixtures_and_criteria = [
            ("showcase_pipeline.R", "df2$total"),
            ("partial_retention.R", "df2$total"),
            ("filter_provenance.R", "df3$total"),
            ("uncertainty.R", "df2$dynamic_col"),
        ];

        for (fix, crit) in &fixtures_and_criteria {
            let path = find_fixture(fix);
            let mut text_outputs = Vec::new();
            let mut json_outputs = Vec::new();

            for _ in 0..5 {
                let (_, out_text) = run_cli(vec![
                    "rparse".to_string(),
                    "slice".to_string(),
                    path.to_str().unwrap().to_string(),
                    "--criterion".to_string(),
                    crit.to_string(),
                ]);
                let (_, out_json) = run_cli(vec![
                    "rparse".to_string(),
                    "slice".to_string(),
                    path.to_str().unwrap().to_string(),
                    "--criterion".to_string(),
                    crit.to_string(),
                    "--format".to_string(),
                    "json".to_string(),
                ]);
                text_outputs.push(out_text);
                json_outputs.push(out_json);
            }

            for i in 1..5 {
                assert_eq!(
                    text_outputs[0], text_outputs[i],
                    "Non-deterministic text slice for {}",
                    fix
                );
                assert_eq!(
                    json_outputs[0], json_outputs[i],
                    "Non-deterministic json slice for {}",
                    fix
                );
            }
        }
    }
}

// ============================================================================
// 12. PHASE C MILESTONE 3: INTERACTIVE ANALYSIS VISUALIZER TESTS
// ============================================================================

mod milestone3_visualizer_tests {
    use super::*;

    fn find_fixture(name: &str) -> PathBuf {
        let mut dir = std::env::current_dir().unwrap();
        loop {
            let candidate = dir.join("fixtures").join(name);
            if candidate.exists() {
                return candidate;
            }
            let candidate2 = dir.join("implementation").join("fixtures").join(name);
            if candidate2.exists() {
                return candidate2;
            }
            if !dir.pop() {
                panic!("Could not find fixture {}", name);
            }
        }
    }

    fn extract_embedded_json(html: &str) -> JsonValue {
        let start_tag = "<script type=\"application/json\" id=\"rparse-report-data\">";
        let end_tag = "</script>";
        let start_idx = html
            .find(start_tag)
            .expect("HTML must contain rparse-report-data script tag")
            + start_tag.len();
        let end_idx = html[start_idx..]
            .find(end_tag)
            .expect("script tag must have closing </script>")
            + start_idx;
        let json_body = html[start_idx..end_idx].trim();
        JsonValue::parse(json_body).expect("Embedded data must be valid JSON")
    }

    #[test]
    fn test_milestone3_html_generation_from_fixture() {
        let path = find_fixture("showcase_pipeline.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS, "HTML report generation failed: {}", out);
        assert!(out.contains("<!DOCTYPE html>"));
        assert!(out.contains("R-PARSE Workbench"));
        assert!(out.contains("<script type=\"application/json\" id=\"rparse-report-data\">"));
        assert!(out.contains("renderSourceViewer()"));
        assert!(out.contains("renderProvenanceGraph()"));
        assert!(out.contains("renderPdgGraph()"));
        assert!(out.contains("renderSliceInspector()"));
        assert!(out.contains("renderSchemaEvolution()"));

        let data = extract_embedded_json(&out);
        assert_eq!(data.get("tool").unwrap().as_str(), Some("rparse"));
        assert_eq!(
            data.get("file_name").unwrap().as_str(),
            Some("showcase_pipeline.R")
        );
        assert_eq!(data.get("mode").unwrap().as_str(), Some("STATIC"));
    }

    #[test]
    fn test_milestone3_graph_nodes_and_edges_included() {
        let path = find_fixture("showcase_pipeline.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        let data = extract_embedded_json(&out);

        // PDG topology verification
        let pdg = data.get("pdg").expect("pdg must exist");
        let nodes = pdg.get("nodes").unwrap().as_array().unwrap();
        let edges = pdg.get("edges").unwrap().as_array().unwrap();
        assert!(nodes.len() > 15, "PDG must contain nodes");
        assert!(edges.len() > 15, "PDG must contain edges");

        // Verify representative node kinds exist
        let has_stmt = nodes
            .iter()
            .any(|n| n.get("kind").unwrap().as_str() == Some("Statement"));
        let has_expr = nodes
            .iter()
            .any(|n| n.get("kind").unwrap().as_str() == Some("Expression"));
        let has_col = nodes
            .iter()
            .any(|n| n.get("kind").unwrap().as_str() == Some("ColumnVersion"));
        assert!(has_stmt, "PDG must contain Statement nodes");
        assert!(has_expr, "PDG must contain Expression nodes");
        assert!(has_col, "PDG must contain ColumnVersion nodes");

        // Provenance verification
        let prov = data.get("provenance").expect("provenance must exist");
        let prov_nodes = prov.get("nodes").unwrap().as_array().unwrap();
        let prov_edges = prov.get("edges").unwrap().as_array().unwrap();
        assert!(!prov_nodes.is_empty(), "Provenance must contain nodes");
        assert!(!prov_edges.is_empty(), "Provenance must contain edges");
    }

    #[test]
    fn test_milestone3_grouped_multi_input_provenance() {
        let path = find_fixture("showcase_pipeline.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        let data = extract_embedded_json(&out);
        let prov = data.get("provenance").expect("provenance exists");
        let derivations = prov.get("derivations").unwrap().as_array().unwrap();

        // total = price * quantity has 2 input columns: price and quantity
        let multi_input_deriv = derivations
            .iter()
            .find(|d| {
                let in_cols = d.get("input_columns").unwrap().as_array().unwrap();
                in_cols.len() >= 2
            })
            .expect("Must contain grouped multi-input derivation for total = price * quantity");

        assert_eq!(
            multi_input_deriv.get("knowledge").unwrap().as_str(),
            Some("KNOWN_STATIC")
        );
        assert_eq!(
            multi_input_deriv.get("is_unknown").unwrap().as_bool(),
            Some(false)
        );

        // In generic edges, multiple input_column edges point to this pedge
        let deriv_id = multi_input_deriv.get("id").unwrap().as_u64().unwrap();
        let expected_node_id = format!("pedge_{}", deriv_id);
        let prov_edges = prov.get("edges").unwrap().as_array().unwrap();
        let incoming_to_pedge: Vec<_> = prov_edges
            .iter()
            .filter(|e| e.get("to").unwrap().as_str() == Some(&expected_node_id))
            .collect();
        assert!(
            incoming_to_pedge.len() >= 2,
            "Derivation node must receive edges from multiple input columns"
        );
    }

    #[test]
    fn test_milestone3_row_selection_vs_control_dependency() {
        let path = find_fixture("filter_provenance.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        let data = extract_embedded_json(&out);

        // PDG edges distinction
        let pdg = data.get("pdg").expect("pdg exists");
        let edges = pdg.get("edges").unwrap().as_array().unwrap();
        let row_sel_edges: Vec<_> = edges
            .iter()
            .filter(|e| e.get("kind").unwrap().as_str() == Some("RowSelectionDependency"))
            .collect();
        assert!(
            !row_sel_edges.is_empty(),
            "Must contain RowSelectionDependency edges"
        );

        let ctrl_edges: Vec<_> = edges
            .iter()
            .filter(|e| e.get("kind").unwrap().as_str() == Some("ControlDependency"))
            .collect();
        assert!(
            ctrl_edges.is_empty(),
            "Sequential filters must not introduce false CFG ControlDependency edges"
        );

        // Schema evolution row selections
        let schema_evo = data.get("schema_evolution").unwrap().as_array().unwrap();
        assert!(schema_evo.len() >= 3);
        let has_row_sel_state = schema_evo.iter().any(|st| {
            let rs = st.get("row_selections").unwrap().as_array().unwrap();
            !rs.is_empty()
        });
        assert!(
            has_row_sel_state,
            "Schema evolution must record row selections on filtered dataframe states"
        );
    }

    #[test]
    fn test_milestone3_slice_retention_sub_elements_and_spans() {
        let path = find_fixture("partial_retention.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        let data = extract_embedded_json(&out);
        let slices = data.get("slices").unwrap().as_object().unwrap();

        let total_slice = slices
            .get("df2$total")
            .expect("Must contain precomputed slice for df2$total");
        assert_eq!(
            total_slice.get("confidence").unwrap().as_str(),
            Some("COMPLETE")
        );
        assert_eq!(
            total_slice.get("contains_unknowns").unwrap().as_bool(),
            Some(false)
        );

        let stmts = total_slice.get("statements").unwrap().as_array().unwrap();
        // Line 21 is the mutate statement with 2 sub-elements (total [KEEP], unused [DROP])
        let partial_stmt = stmts
            .iter()
            .find(|s| {
                s.get("start_line").and_then(|l| l.as_u64()) == Some(21)
                    && s.get("state").unwrap().as_str() == Some("partially_retained")
            })
            .expect("Must contain partially_retained mutate statement at line 21");

        let sub_elements = partial_stmt
            .get("sub_elements")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(sub_elements.len(), 2);

        let kept = &sub_elements[0];
        assert_eq!(kept.get("state").unwrap().as_str(), Some("retained"));
        assert_eq!(
            kept.get("label").unwrap().as_str(),
            Some("total = price * quantity")
        );
        let kept_span = kept.get("span").unwrap();
        assert_eq!(kept_span.get("start_line").unwrap().as_u64(), Some(21));

        let dropped = &sub_elements[1];
        assert_eq!(dropped.get("state").unwrap().as_str(), Some("excluded"));
        assert_eq!(
            dropped.get("label").unwrap().as_str(),
            Some("unused = age * 2")
        );
        let dropped_span = dropped.get("span").unwrap();
        assert_eq!(dropped_span.get("start_line").unwrap().as_u64(), Some(21));
    }

    #[test]
    fn test_milestone3_uncertainty_explicit_preservation() {
        let path = find_fixture("uncertainty.R");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        let data = extract_embedded_json(&out);

        // Summary uncertainty flag
        let summary = data.get("summary").unwrap();
        assert_eq!(
            summary.get("contains_unknowns").unwrap().as_bool(),
            Some(true)
        );

        // Slice for dynamic column retains uncertainty
        let slices = data.get("slices").unwrap().as_object().unwrap();
        let dynamic_slice = slices
            .get("df2$dynamic_col")
            .expect("Must contain precomputed slice for df2$dynamic_col");
        assert_eq!(
            dynamic_slice.get("confidence").unwrap().as_str(),
            Some("PARTIAL_CONTAINS_UNKNOWNS")
        );
        assert_eq!(
            dynamic_slice.get("contains_unknowns").unwrap().as_bool(),
            Some(true)
        );

        // Provenance uncertainty
        let prov = data.get("provenance").unwrap();
        let derivations = prov.get("derivations").unwrap().as_array().unwrap();
        let unk_deriv = derivations
            .iter()
            .find(|d| d.get("is_unknown").unwrap().as_bool() == Some(true))
            .expect("Must contain explicit unknown derivation");
        assert_eq!(
            unk_deriv.get("knowledge").unwrap().as_str(),
            Some("UNKNOWN_STATIC")
        );
    }

    #[test]
    fn test_milestone3_source_escaping_and_safe_json_embedding() {
        let malicious_code = "# Injected script: </script><script>alert('xss')</script>\nx <- \"hello <world> & 'foo'\"\n";
        let script_path = create_temp_script("xss_test", malicious_code);

        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);

        // Critical safety assertion: </script> must NEVER appear inside the embedded script body!
        let start_tag = "<script type=\"application/json\" id=\"rparse-report-data\">";
        let start_pos = out.find(start_tag).unwrap() + start_tag.len();
        let end_pos = out[start_pos..].find("</script>").unwrap() + start_pos;
        let script_body = &out[start_pos..end_pos];

        assert!(
            !script_body.contains("</script>"),
            "JSON script body must safely escape </script>"
        );
        assert!(
            !script_body.contains("<script>"),
            "JSON script body must safely escape <script>"
        );

        // Verifying standard JSON parser decodes back to exact string
        let data = extract_embedded_json(&out);
        let src = data.get("source_code").unwrap().as_str().unwrap();
        assert!(src.contains("</script><script>alert('xss')</script>"));
        assert!(src.contains("\"hello <world> & 'foo'\""));
    }

    #[test]
    fn test_milestone3_deterministic_report_output() {
        let path = find_fixture("showcase_pipeline.R");
        let mut reports = Vec::new();

        for _ in 0..5 {
            let (code, out) = run_cli(vec![
                "rparse".to_string(),
                "analyze".to_string(),
                path.to_str().unwrap().to_string(),
                "--format".to_string(),
                "html".to_string(),
            ]);
            assert_eq!(code, EXIT_SUCCESS);
            reports.push(out);
        }

        for i in 1..5 {
            assert_eq!(
                reports[0], reports[i],
                "HTML report generation must be byte-for-byte deterministic"
            );
        }
    }

    #[test]
    fn test_milestone3_empty_input_and_graceful_handling() {
        let empty_path = create_temp_script("empty_bench", "");
        let (code, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            empty_path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code, EXIT_SUCCESS);
        assert!(out.contains("<!DOCTYPE html>"));
        let data = extract_embedded_json(&out);
        assert_eq!(
            data.get("summary")
                .unwrap()
                .get("statement_count")
                .unwrap()
                .as_u64(),
            Some(0)
        );
    }

    #[test]
    fn test_milestone3_cli_output_file_writing_and_error() {
        let path = find_fixture("showcase_pipeline.R");
        let temp_dir = std::env::temp_dir();
        let target_file = temp_dir.join(format!("rparse_test_m3_{}.html", std::process::id()));

        // Successful write with -o
        let (code_ok, out_ok) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
            "-o".to_string(),
            target_file.to_str().unwrap().to_string(),
        ]);
        assert_eq!(code_ok, EXIT_SUCCESS);
        assert!(target_file.exists());
        let file_content = fs::read_to_string(&target_file).expect("read written html report");
        assert_eq!(file_content, out_ok);
        assert!(file_content.contains("<!DOCTYPE html>"));
        let _ = fs::remove_file(&target_file);

        // Invalid output path error handling. The parent directory must be
        // absent on every OS: `Z:\...` is one legal filename on Unix, where
        // `\` is not a separator, so that write succeeds.
        let invalid_target = std::env::temp_dir()
            .join(format!(
                "rparse_nonexistent_invalid_dir_{}",
                std::process::id()
            ))
            .join("report.html");
        let (code_err, out_err) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "html".to_string(),
            "--output".to_string(),
            invalid_target.to_str().unwrap().to_string(),
        ]);
        assert_ne!(code_err, EXIT_SUCCESS);
        assert!(out_err.contains("Failed to write output file"));

        // Backward compatibility: default analyze text remains identical
        let (code_text, out_text) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
        ]);
        assert_eq!(code_text, EXIT_SUCCESS);
        assert!(out_text.contains("R-PARSE STATIC ANALYSIS"));

        // Backward compatibility: analyze json remains identical
        let (code_json, out_json) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_json, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out_json).expect("valid analyze json");
        assert_eq!(parsed.get("command").unwrap().as_str(), Some("analyze"));
    }
}

mod dataframe_aliasing_tests {
    use super::*;

    fn find_fixture(name: &str) -> PathBuf {
        let mut dir = std::env::current_dir().unwrap();
        loop {
            let candidate = dir.join("fixtures").join(name);
            if candidate.exists() {
                return candidate;
            }
            let candidate2 = dir.join("implementation").join("fixtures").join(name);
            if candidate2.exists() {
                return candidate2;
            }
            if !dir.pop() {
                panic!("Could not find fixture {}", name);
            }
        }
    }

    #[test]
    fn test_1_simple_dataframe_alias() {
        let code = "df <- data.frame(price = 100, quantity = 5, noise = 99)\ndf_alias <- df\n";
        let script_path = create_temp_script("alias_simple", code);

        // Analyze JSON
        let (code_val, out) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_val, EXIT_SUCCESS);
        let _parsed = JsonValue::parse(&out).expect("valid analyze json");

        // Slice directly on df_alias$price
        let (slice_code, slice_out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df_alias$price".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(slice_code, EXIT_SUCCESS);
        let slice_json = JsonValue::parse(&slice_out).expect("valid slice json");
        assert_eq!(
            slice_json.get("confidence").unwrap().as_str(),
            Some("COMPLETE")
        );

        // Verify df_alias <- df is fully retained and df has price retained and noise excluded
        let stmts = slice_json.get("statements").unwrap().as_array().unwrap();
        let alias_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(2));
        assert!(
            alias_stmt.is_some(),
            "df_alias <- df (line 2) must be in statements"
        );
        assert_eq!(
            alias_stmt.unwrap().get("state").unwrap().as_str(),
            Some("fully_retained")
        );

        let df_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(1))
            .unwrap();
        assert_eq!(
            df_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let sub_elements = df_stmt.get("sub_elements").unwrap().as_array().unwrap();
        let price_elem = sub_elements
            .iter()
            .find(|r| r.get("label").unwrap().as_str().unwrap().contains("price"))
            .unwrap();
        assert_eq!(price_elem.get("state").unwrap().as_str(), Some("retained"));
        let noise_elem = sub_elements
            .iter()
            .find(|r| r.get("label").unwrap().as_str().unwrap().contains("noise"))
            .unwrap();
        assert_eq!(noise_elem.get("state").unwrap().as_str(), Some("excluded"));
    }

    #[test]
    fn test_2_alias_followed_by_derived_column_provenance() {
        let code = "df <- data.frame(price = 100, quantity = 5, noise = 99)\ndf_alias <- df\ndf_out <- mutate(df_alias, total = price * quantity)\n";
        let script_path = create_temp_script("alias_derived_prov", code);

        let (code_val, out) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--column".to_string(),
            "df_out$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_val, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid provenance json");
        assert_eq!(parsed.get("is_unknown").unwrap().as_bool(), Some(false));

        let direct_inputs = parsed.get("direct_inputs").unwrap().as_array().unwrap();
        let direct_names: Vec<&str> = direct_inputs
            .iter()
            .map(|c| c.get("column").unwrap().as_str().unwrap())
            .collect();
        assert!(direct_names.contains(&"price"));
        assert!(direct_names.contains(&"quantity"));
        assert!(!direct_names.contains(&"noise"));

        let transitive_inputs = parsed.get("transitive_inputs").unwrap().as_array().unwrap();
        let trans_names: Vec<&str> = transitive_inputs
            .iter()
            .map(|c| c.get("column").unwrap().as_str().unwrap())
            .collect();
        assert!(trans_names.contains(&"price"));
        assert!(trans_names.contains(&"quantity"));
        assert!(!trans_names.contains(&"noise"));
    }

    #[test]
    fn test_3_backward_slicing_through_alias_sub_statement() {
        let code = "df <- data.frame(price = 100, quantity = 5, noise = 99)\ndf_alias <- df\ndf_out <- mutate(df_alias, total = price * quantity, unused = noise * 2)\n";
        let script_path = create_temp_script("alias_slice_sub", code);

        let (code_val, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df_out$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_val, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid slice json");
        assert_eq!(parsed.get("confidence").unwrap().as_str(), Some("COMPLETE"));
        assert_eq!(
            parsed.get("contains_unknowns").unwrap().as_bool(),
            Some(false)
        );

        let stmts = parsed.get("statements").unwrap().as_array().unwrap();

        // Check data.frame statement (line 1): price and quantity retained, noise excluded
        let df_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(1))
            .unwrap();
        assert_eq!(
            df_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let df_subs = df_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(
            df_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("price"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            df_subs
                .iter()
                .find(|r| r
                    .get("label")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("quantity"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            df_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("noise"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("excluded")
        );

        // Check alias statement (line 2): fully_retained
        let alias_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(2))
            .unwrap();
        assert_eq!(
            alias_stmt.get("state").unwrap().as_str(),
            Some("fully_retained")
        );

        // Check mutate statement (line 3): total retained, unused excluded
        let mutate_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(3))
            .unwrap();
        assert_eq!(
            mutate_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let mutate_subs = mutate_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(
            mutate_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("total"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            mutate_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("unused"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("excluded")
        );
    }

    #[test]
    fn test_4_two_independent_dataframes_with_identically_named_columns() {
        let code = "df1 <- data.frame(val = 10, other = 1)\ndf2 <- data.frame(val = 20, other = 2)\ndf1_alias <- df1\nout <- mutate(df1_alias, res = val * 2)\n";
        let script_path = create_temp_script("alias_two_dfs", code);

        let (code_val, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "out$res".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_val, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid slice json");

        let stmts = parsed.get("statements").unwrap().as_array().unwrap();
        // df1 (line 1) must be retained with val retained, other excluded
        let df1_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(1))
            .unwrap();
        assert_eq!(
            df1_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let df1_subs = df1_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(
            df1_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("val"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            df1_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("other"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("excluded")
        );

        // df2 (line 2) must be not_retained in the slice
        let df2_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(2))
            .unwrap();
        assert_eq!(
            df2_stmt.get("state").unwrap().as_str(),
            Some("not_retained")
        );
    }

    #[test]
    fn test_5_reassignment_after_alias_creation_no_conflation() {
        let code = "df <- data.frame(x = 10, y = 20)\ndf_alias <- df\ndf <- mutate(df, x = 999)\ndf_out <- mutate(df_alias, total = x * 2)\n";
        let script_path = create_temp_script("alias_reassign_order", code);

        let (code_val, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            script_path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df_out$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_val, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid slice json");

        let stmts = parsed.get("statements").unwrap().as_array().unwrap();

        // Initial df constructor (line 1) retained with x retained, y excluded
        let init_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(1))
            .unwrap();
        assert_eq!(
            init_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let init_subs = init_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(
            init_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("x"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            init_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("y"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("excluded")
        );

        // Alias statement (line 2) fully_retained
        let alias_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(2))
            .unwrap();
        assert_eq!(
            alias_stmt.get("state").unwrap().as_str(),
            Some("fully_retained")
        );

        // Subsequent reassignment df <- mutate(df, x = 999) (line 3) must be not_retained
        let reassign_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(3))
            .unwrap();
        assert_eq!(
            reassign_stmt.get("state").unwrap().as_str(),
            Some("not_retained")
        );
    }

    #[test]
    fn test_6_adversarial_adv04_alias_rename_full_pass() {
        let path = find_fixture("adversarial/adv04_alias_rename.R");

        let (code_val, out) = run_cli(vec![
            "rparse".to_string(),
            "slice".to_string(),
            path.to_str().unwrap().to_string(),
            "--criterion".to_string(),
            "df_final$final_score".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code_val, EXIT_SUCCESS);
        let parsed = JsonValue::parse(&out).expect("valid slice json");

        assert_eq!(parsed.get("confidence").unwrap().as_str(), Some("COMPLETE"));
        assert_eq!(
            parsed.get("contains_unknowns").unwrap().as_bool(),
            Some(false)
        );

        let stmts = parsed.get("statements").unwrap().as_array().unwrap();
        // Line 4: df <- data.frame(raw_score = 100, noise = 50)
        let df_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(4))
            .unwrap();
        assert_eq!(
            df_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let df_subs = df_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(
            df_subs
                .iter()
                .find(|r| r
                    .get("label")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("raw_score"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            df_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("noise"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("excluded")
        );

        // Line 5: df_alias <- df
        let alias_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(5))
            .unwrap();
        assert_eq!(
            alias_stmt.get("state").unwrap().as_str(),
            Some("fully_retained")
        );

        // Line 6: df_renamed <- rename(df_alias, score = raw_score)
        let rename_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(6))
            .unwrap();
        assert_eq!(
            rename_stmt.get("state").unwrap().as_str(),
            Some("fully_retained")
        );

        // Line 7: df_final <- mutate(df_renamed, final_score = score + 10, unused = noise * 2)
        let final_stmt = stmts
            .iter()
            .find(|s| s.get("start_line").unwrap().as_u64() == Some(7))
            .unwrap();
        assert_eq!(
            final_stmt.get("state").unwrap().as_str(),
            Some("partially_retained")
        );
        let final_subs = final_stmt.get("sub_elements").unwrap().as_array().unwrap();
        assert_eq!(
            final_subs
                .iter()
                .find(|r| r
                    .get("label")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("final_score"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("retained")
        );
        assert_eq!(
            final_subs
                .iter()
                .find(|r| r.get("label").unwrap().as_str().unwrap().contains("unused"))
                .unwrap()
                .get("state")
                .unwrap()
                .as_str(),
            Some("excluded")
        );
    }
}

// ============================================================================
// 12. REAL ANALYSIS INTEGRATION TESTS (MISSION SCENARIOS A - H)
// ============================================================================

mod real_analysis_integration_tests {
    use super::*;
    use crate::server::handle_api_analyze;
    use rparse_core::JsonValue;

    #[test]
    fn test_scenario_a_basic_arithmetic_transformation() {
        // Program 1: Multiplication
        let p1 = r#"
df <- data.frame(price = c(10, 20), quantity = c(2, 3))
df$total <- df$price * df$quantity
"#;
        let payload1 = format!(
            r#"{{"source": {}, "revision": 1, "file_name": "arith1.R"}}"#,
            JsonValue::String(p1.to_string()).to_json_string()
        );
        let (status1, resp1) = handle_api_analyze(&payload1);
        assert_eq!(status1, 200);
        let res1 = JsonValue::parse(&resp1).expect("valid json response 1");
        assert_eq!(res1.get("revision").unwrap().as_u64(), Some(1));
        assert_eq!(res1.get("analysis_status").unwrap().as_str(), Some("ok"));

        // Program 2: Addition
        let p2 = r#"
df <- data.frame(price = c(10, 20), quantity = c(2, 3))
df$total <- df$price + df$quantity
"#;
        let payload2 = format!(
            r#"{{"source": {}, "revision": 2, "file_name": "arith2.R"}}"#,
            JsonValue::String(p2.to_string()).to_json_string()
        );
        let (status2, resp2) = handle_api_analyze(&payload2);
        assert_eq!(status2, 200);
        let res2 = JsonValue::parse(&resp2).expect("valid json response 2");
        assert_eq!(res2.get("revision").unwrap().as_u64(), Some(2));
        assert_eq!(res2.get("analysis_status").unwrap().as_str(), Some("ok"));

        // Verify that expressions differ between revisions
        let prov1_edges = res1
            .get("provenance")
            .unwrap()
            .get("edges")
            .unwrap()
            .as_array()
            .unwrap();
        let prov2_edges = res2
            .get("provenance")
            .unwrap()
            .get("edges")
            .unwrap()
            .as_array()
            .unwrap();
        assert!(
            !prov1_edges.is_empty(),
            "Program 1 must have provenance edges"
        );
        assert!(
            !prov2_edges.is_empty(),
            "Program 2 must have provenance edges"
        );

        let lines1 = res1.get("source_lines").unwrap().as_array().unwrap();
        let lines2 = res2.get("source_lines").unwrap().as_array().unwrap();
        let total_line_1 = lines1
            .iter()
            .find(|l| l.as_str().unwrap().contains("df$total"))
            .unwrap()
            .as_str()
            .unwrap();
        let total_line_2 = lines2
            .iter()
            .find(|l| l.as_str().unwrap().contains("df$total"))
            .unwrap()
            .as_str()
            .unwrap();

        assert!(
            total_line_1.contains('*'),
            "Program 1 must reflect multiplication expression: {}",
            total_line_1
        );
        assert!(
            total_line_2.contains('+'),
            "Program 2 must reflect addition expression: {}",
            total_line_2
        );
        assert_ne!(
            total_line_1, total_line_2,
            "Derivation statements must differ between * and +"
        );
    }

    #[test]
    fn test_scenario_b_renamed_output_column() {
        // Program 1: output column is 'total'
        let p1 = r#"
df <- data.frame(price = c(10, 20), quantity = c(2, 3))
df$total <- df$price * df$quantity
"#;
        let payload1 = format!(
            r#"{{"source": {}, "revision": 1}}"#,
            JsonValue::String(p1.to_string()).to_json_string()
        );
        let (status1, resp1) = handle_api_analyze(&payload1);
        assert_eq!(status1, 200);
        let res1 = JsonValue::parse(&resp1).unwrap();

        let schema1 = res1.get("schema_evolution").unwrap().as_array().unwrap();
        let has_total_1 = schema1.iter().any(|st| {
            st.get("columns")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c.get("name").unwrap().as_str() == Some("total"))
        });
        assert!(has_total_1, "Program 1 must contain column 'total'");

        // Program 2: output column is renamed to 'revenue'
        let p2 = r#"
df <- data.frame(price = c(10, 20), quantity = c(2, 3))
df$revenue <- df$price * df$quantity
"#;
        let payload2 = format!(
            r#"{{"source": {}, "revision": 2}}"#,
            JsonValue::String(p2.to_string()).to_json_string()
        );
        let (status2, resp2) = handle_api_analyze(&payload2);
        assert_eq!(status2, 200);
        let res2 = JsonValue::parse(&resp2).unwrap();

        let schema2 = res2.get("schema_evolution").unwrap().as_array().unwrap();
        let has_revenue_2 = schema2.iter().any(|st| {
            st.get("columns")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c.get("name").unwrap().as_str() == Some("revenue"))
        });
        let has_total_2 = schema2.iter().any(|st| {
            st.get("columns")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c.get("name").unwrap().as_str() == Some("total"))
        });

        assert!(has_revenue_2, "Program 2 must contain column 'revenue'");
        assert!(
            !has_total_2,
            "Program 2 must NOT retain obsolete column 'total'"
        );
    }

    #[test]
    fn test_scenario_c_unrelated_column_and_backward_slice() {
        let code = r#"
df <- data.frame(price = c(10, 20), quantity = c(2, 3), unrelated = c(99, 99))
df$total <- df$price * df$quantity
df$extra <- df$unrelated + 1
"#;
        let payload = format!(
            r#"{{"source": {}, "criterion": "df$total"}}"#,
            JsonValue::String(code.to_string()).to_json_string()
        );
        let (status, resp) = handle_api_analyze(&payload);
        assert_eq!(status, 200);
        let res = JsonValue::parse(&resp).unwrap();

        let slices = res.get("slices").unwrap().as_object().unwrap();
        let slice = slices
            .get("df$total")
            .expect("slice for df$total must be computed");
        let statements = slice.get("statements").unwrap().as_array().unwrap();

        // The statement computing df$extra (line 4) must NOT be fully retained for df$total slice
        let extra_stmt = statements.iter().find(|s| {
            s.get("code")
                .map(|c| c.as_str().unwrap().contains("extra"))
                .unwrap_or(false)
        });
        if let Some(s) = extra_stmt {
            let state = s.get("state").unwrap().as_str().unwrap();
            assert_ne!(
                state, "fully_retained",
                "Unrelated statement must not be fully retained"
            );
        }
    }

    #[test]
    fn test_scenario_d_invalid_syntax() {
        let malformed = "df <- data.frame(price = \n";
        let payload = format!(
            r#"{{"source": {}, "revision": 42}}"#,
            JsonValue::String(malformed.to_string()).to_json_string()
        );
        let (status, resp) = handle_api_analyze(&payload);
        assert_eq!(status, 200);
        let res = JsonValue::parse(&resp).unwrap();

        // Must report syntax_error or error
        let analysis_status = res.get("analysis_status").unwrap().as_str().unwrap();
        assert!(
            analysis_status == "syntax_error" || analysis_status == "error",
            "Status must be syntax_error or error: {}",
            analysis_status
        );
        // Source must be preserved exactly
        assert_eq!(res.get("source_code").unwrap().as_str(), Some(malformed));
        // Diagnostics must contain error
        let diags = res.get("diagnostics").unwrap().as_array().unwrap();
        assert!(!diags.is_empty(), "Must emit diagnostics on invalid syntax");
        assert!(diags
            .iter()
            .any(|d| d.get("severity").unwrap().as_str() == Some("error")));
    }

    #[test]
    fn test_scenario_e_unsupported_or_dynamic_operation() {
        let code = r#"
df <- data.frame(price = 10, quantity = 5)
df2 <- mutate(df, dynamic_col = .data[[get_col_name()]])
"#;
        let payload = format!(
            r#"{{"source": {}}}"#,
            JsonValue::String(code.to_string()).to_json_string()
        );
        let (status, resp) = handle_api_analyze(&payload);
        assert_eq!(status, 200);
        let res = JsonValue::parse(&resp).unwrap();

        let contains_unk = res
            .get("summary")
            .unwrap()
            .get("contains_unknowns")
            .unwrap()
            .as_bool()
            .unwrap();
        assert!(
            contains_unk,
            "Dynamic column indexing must preserve UNKNOWN_STATIC uncertainty"
        );
        assert_eq!(
            res.get("analysis_status").unwrap().as_str(),
            Some("uncertain")
        );
    }

    #[test]
    fn test_scenario_f_rapid_successive_analysis_revisions() {
        let p1 = "x <- 1\n";
        let p2 = "x <- 2\n";

        let (status1, resp1) = handle_api_analyze(&format!(
            r#"{{"source": {}, "revision": 100}}"#,
            JsonValue::String(p1.to_string()).to_json_string()
        ));
        let (status2, resp2) = handle_api_analyze(&format!(
            r#"{{"source": {}, "revision": 101}}"#,
            JsonValue::String(p2.to_string()).to_json_string()
        ));

        assert_eq!(status1, 200);
        assert_eq!(status2, 200);

        let res1 = JsonValue::parse(&resp1).unwrap();
        let res2 = JsonValue::parse(&resp2).unwrap();

        assert_eq!(res1.get("revision").unwrap().as_u64(), Some(100));
        assert_eq!(res2.get("revision").unwrap().as_u64(), Some(101));
        assert_eq!(res1.get("source_code").unwrap().as_str(), Some(p1));
        assert_eq!(res2.get("source_code").unwrap().as_str(), Some(p2));
    }

    #[test]
    fn test_scenario_g_empty_source_and_no_dataframes() {
        // Empty source
        let (status_empty, resp_empty) = handle_api_analyze(r#"{"source": "   \n\t  "}"#);
        assert_eq!(status_empty, 200);
        let res_empty = JsonValue::parse(&resp_empty).unwrap();
        assert_eq!(
            res_empty.get("analysis_status").unwrap().as_str(),
            Some("empty_source")
        );

        // No dataframes, just scalars
        let scalar_code = "a <- 1\nb <- a + 2\n";
        let (status_sc, resp_sc) = handle_api_analyze(&format!(
            r#"{{"source": {}}}"#,
            JsonValue::String(scalar_code.to_string()).to_json_string()
        ));
        assert_eq!(status_sc, 200);
        let res_sc = JsonValue::parse(&resp_sc).unwrap();
        assert_eq!(res_sc.get("analysis_status").unwrap().as_str(), Some("ok"));
        let prov_nodes = res_sc
            .get("provenance")
            .unwrap()
            .get("nodes")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(
            prov_nodes.len(),
            0,
            "No dataframe column provenance nodes should exist"
        );
        let pdg_nodes = res_sc
            .get("pdg")
            .unwrap()
            .get("nodes")
            .unwrap()
            .as_array()
            .unwrap();
        assert!(pdg_nodes.len() >= 2, "PDG should contain scalar statements");
    }

    #[test]
    fn test_scenario_h_cli_compatibility() {
        let temp_script = create_temp_script(
            "showcase_compat",
            include_str!("../../../fixtures/showcase_pipeline.R"),
        );
        let script_str = temp_script.to_str().unwrap().to_string();

        // 1. Analyze with text format
        let (code1, out1) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            script_str.clone(),
        ]);
        assert_eq!(code1, EXIT_SUCCESS);
        assert!(out1.contains("Statements:"));

        // 2. Analyze with json format
        let (code2, out2) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            script_str.clone(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code2, EXIT_SUCCESS);
        let parsed2 = JsonValue::parse(&out2).expect("valid analyze json");
        assert_eq!(parsed2.get("tool").unwrap().as_str(), Some("rparse"));

        // 3. Analyze with html format
        let (code3, out3) = run_cli(vec![
            "rparse".to_string(),
            "analyze".to_string(),
            script_str.clone(),
            "--format".to_string(),
            "html".to_string(),
        ]);
        assert_eq!(code3, EXIT_SUCCESS);
        assert!(out3.contains("<!DOCTYPE html>"));
        assert!(out3.contains("id=\"rparse-report-data\""));

        // 4. Provenance command
        let (code4, out4) = run_cli(vec![
            "rparse".to_string(),
            "provenance".to_string(),
            script_str.clone(),
            "--column".to_string(),
            "df2$total".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        assert_eq!(code4, EXIT_SUCCESS);
        let parsed4 = JsonValue::parse(&out4).expect("valid provenance json");
        assert!(parsed4.get("target").is_some());

        // 5. Studio help command
        let (code5, out5) = run_cli(vec![
            "rparse".to_string(),
            "studio".to_string(),
            "--help".to_string(),
        ]);
        assert_eq!(code5, EXIT_SUCCESS);
        assert!(out5.contains("rparse studio"));
    }

    #[test]
    fn test_loopback_http_server_endpoints() {
        use std::io::{BufRead, BufReader, Read, Write};
        use std::net::{TcpListener, TcpStream};
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use std::thread;
        use std::time::Duration;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let port = listener.local_addr().unwrap().port();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            let initial_html = "<html><body>Studio Test</body></html>".to_string();
            listener.set_nonblocking(true).unwrap();
            while running_clone.load(Ordering::Relaxed) {
                if let Ok((mut stream, _)) = listener.accept() {
                    // macOS accept() copies the listener's O_NONBLOCK flag.
                    // A nonblocking read then returns WouldBlock, the socket
                    // is dropped, and the client sees ECONNRESET.
                    if stream.set_nonblocking(false).is_err() {
                        continue;
                    }
                    let mut reader = BufReader::new(&mut stream);
                    let mut line = String::new();
                    if reader.read_line(&mut line).is_ok() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let method = parts[0];
                            let path = parts[1];
                            // Read the remaining headers before responding.
                            // Closing with unread request bytes resets the
                            // peer on macOS.
                            let mut content_length = 0usize;
                            loop {
                                let mut header = String::new();
                                match reader.read_line(&mut header) {
                                    Ok(0) => break,
                                    Ok(_) if header.trim().is_empty() => break,
                                    Ok(_) => {
                                        if let Some((k, v)) = header.split_once(':') {
                                            if k.trim().eq_ignore_ascii_case("content-length") {
                                                content_length = v.trim().parse().unwrap_or(0);
                                            }
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }
                            match (method, path) {
                                ("GET", "/api/health") => {
                                    let body =
                                        r#"{"status":"ok","tool":"rparse","version":"0.1.0"}"#;
                                    let resp = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                        body.len(),
                                        body
                                    );
                                    let _ = stream.write_all(resp.as_bytes());
                                }
                                ("GET", "/") => {
                                    let resp = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                        initial_html.len(),
                                        initial_html
                                    );
                                    let _ = stream.write_all(resp.as_bytes());
                                }
                                ("OPTIONS", "/api/analyze") => {
                                    let resp = "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
                                    let _ = stream.write_all(resp.as_bytes());
                                }
                                ("POST", "/api/analyze") => {
                                    let mut body = vec![0u8; content_length];
                                    let _ = reader.read_exact(&mut body);
                                    let body_str = String::from_utf8_lossy(&body);
                                    let (code, json_resp) =
                                        crate::server::handle_api_analyze(&body_str);
                                    let resp = format!(
                                        "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                        code,
                                        json_resp.len(),
                                        json_resp
                                    );
                                    let _ = stream.write_all(resp.as_bytes());
                                }
                                _ => {}
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        // Test 1: GET /api/health over TCP
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        stream
            .write_all(b"GET /api/health HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).unwrap();
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("\"status\":\"ok\""));

        // Test 2: OPTIONS /api/analyze (CORS)
        let mut stream2 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        stream2
            .write_all(b"OPTIONS /api/analyze HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut resp2 = String::new();
        stream2.read_to_string(&mut resp2).unwrap();
        assert!(resp2.starts_with("HTTP/1.1 204 No Content"));
        assert!(resp2.contains("Access-Control-Allow-Origin: *"));

        // Test 3: POST /api/analyze with R code
        let mut stream3 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        let payload = r#"{"source":"df <- data.frame(x=1)\ndf$y <- df$x + 1\n","revision":1}"#;
        let req = format!(
            "POST /api/analyze HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            payload.len(),
            payload
        );
        stream3.write_all(req.as_bytes()).unwrap();
        let mut resp3 = String::new();
        stream3.read_to_string(&mut resp3).unwrap();
        assert!(resp3.starts_with("HTTP/1.1 200 OK"));
        assert!(resp3.contains("\"analysis_status\":\"ok\""));
        assert!(resp3.contains("\"revision\":1"));

        running.store(false, Ordering::Relaxed);
        let _ = handle.join();
    }

    #[test]
    fn test_unified_website_routes_and_same_origin_api() {
        use std::io::{Read, Write};
        use std::net::{TcpListener, TcpStream};
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use std::thread;
        use std::time::Duration;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let port = listener.local_addr().unwrap().port();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            let studio_html = "<html><body>R-PARSE Studio Workspace</body></html>".to_string();
            listener.set_nonblocking(true).unwrap();
            while running_clone.load(Ordering::Relaxed) {
                if let Ok((stream, _)) = listener.accept() {
                    // Same macOS accept() behavior as the loopback endpoint test.
                    if stream.set_nonblocking(false).is_err() {
                        continue;
                    }
                    let studio_clone = studio_html.clone();
                    thread::spawn(move || {
                        let _ = crate::server::handle_connection(stream, &studio_clone);
                    });
                }
                thread::sleep(Duration::from_millis(5));
            }
        });

        // 1. GET / serves the Landing Page HTML
        let mut s1 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s1.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r1 = String::new();
        s1.read_to_string(&mut r1).unwrap();
        assert!(r1.starts_with("HTTP/1.1 200 OK"));
        assert!(r1.contains("text/html"));
        assert!(r1.contains("Every result leaves a trail."));

        // 2. GET /studio serves the Studio Workspace HTML
        let mut s2 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s2.write_all(b"GET /studio HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r2 = String::new();
        s2.read_to_string(&mut r2).unwrap();
        assert!(r2.starts_with("HTTP/1.1 200 OK"));
        assert!(r2.contains("text/html"));
        assert!(r2.contains("R-PARSE Studio Workspace"));

        // 3. GET /api/health serves backend health JSON
        let mut s3 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s3.write_all(b"GET /api/health HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r3 = String::new();
        s3.read_to_string(&mut r3).unwrap();
        assert!(r3.starts_with("HTTP/1.1 200 OK"));
        assert!(r3.contains("application/json"));
        assert!(r3.contains("\"status\":\"ok\""));

        // 4. POST /api/analyze executes Rust static analysis
        let mut s4 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        let payload = r#"{"source":"df <- data.frame(a=1)\n","revision":1}"#;
        let req = format!(
            "POST /api/analyze HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            payload.len(),
            payload
        );
        s4.write_all(req.as_bytes()).unwrap();
        let mut r4 = String::new();
        s4.read_to_string(&mut r4).unwrap();
        assert!(r4.starts_with("HTTP/1.1 200 OK"));
        assert!(r4.contains("\"analysis_status\":\"ok\""));

        // 5. Unknown API route returns 404 JSON, not HTML
        let mut s5 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s5.write_all(b"GET /api/nonexistent HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r5 = String::new();
        s5.read_to_string(&mut r5).unwrap();
        assert!(r5.starts_with("HTTP/1.1 404 Not Found"));
        assert!(r5.contains("application/json"));
        assert!(r5.contains("\"API endpoint not found\""));

        // 6. Missing static JS asset returns 404 text, not HTML
        let mut s6 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s6.write_all(b"GET /missing_script.js HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r6 = String::new();
        s6.read_to_string(&mut r6).unwrap();
        assert!(r6.starts_with("HTTP/1.1 404 Not Found"));
        assert!(r6.contains("text/plain"));
        assert!(r6.contains("404 Asset Not Found"));

        // 7. Unknown page returns 404 text, not HTML
        let mut s7 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s7.write_all(b"GET /unknown_page HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r7 = String::new();
        s7.read_to_string(&mut r7).unwrap();
        assert!(r7.starts_with("HTTP/1.1 404 Not Found"));
        assert!(r7.contains("404 Not Found"));

        // 8. Static assets (CSS) served properly
        let mut s8 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s8.write_all(b"GET /landing/styles.css HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r8 = String::new();
        s8.read_to_string(&mut r8).unwrap();
        assert!(r8.starts_with("HTTP/1.1 200 OK"));
        assert!(r8.contains("text/css"));

        // 9. Favicon SVG route served properly
        let mut s9 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s9.write_all(b"GET /favicon.svg HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r9 = String::new();
        s9.read_to_string(&mut r9).unwrap();
        assert!(r9.starts_with("HTTP/1.1 200 OK"));
        assert!(r9.contains("image/svg+xml"));
        assert!(r9.contains("<svg"));

        // 9b. Raster favicon set (ICO + PNG) served with correct content types
        for (path, mime) in [
            ("/favicon.ico", "image/x-icon"),
            ("/favicon-32x32.png", "image/png"),
            ("/apple-touch-icon.png", "image/png"),
            ("/brand/rparse-mark-96.png", "image/png"),
        ] {
            let mut s = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
            s.write_all(format!("GET {} HTTP/1.1\r\nHost: localhost\r\n\r\n", path).as_bytes())
                .unwrap();
            let mut bytes = Vec::new();
            s.read_to_end(&mut bytes).unwrap();
            let head = String::from_utf8_lossy(&bytes[..bytes.len().min(256)]);
            assert!(head.starts_with("HTTP/1.1 200 OK"), "{path}");
            assert!(head.contains(mime), "{path}");
        }

        // 10. Hero diagram JS asset served properly
        let mut s10 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s10.write_all(b"GET /landing/hero_diagram.js HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r10 = String::new();
        s10.read_to_string(&mut r10).unwrap();
        assert!(r10.starts_with("HTTP/1.1 200 OK"));
        assert!(r10.contains("application/javascript"));

        // 11. Direct query string stripping on routes
        let mut s11 = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        s11.write_all(b"GET /studio?sample=1 HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut r11 = String::new();
        s11.read_to_string(&mut r11).unwrap();
        assert!(r11.starts_with("HTTP/1.1 200 OK"));
        assert!(r11.contains("R-PARSE Studio Workspace"));

        running.store(false, Ordering::Relaxed);
        let _ = handle.join();
    }

    mod ast_explorer_integration_tests {
        use crate::format_ast::build_ast_explorer_payload;
        use crate::pipeline::AnalysisContext;
        use crate::server::handle_api_analyze;
        use rparse_core::JsonValue;
        use rparse_runtime::config::AnalysisMode;
        use std::path::PathBuf;

        #[test]
        fn test_ast_scenario_1_simple_assignment() {
            let code = "x <- 10\n";
            let ctx = AnalysisContext::run_from_source(
                PathBuf::from("test.R"),
                code.to_string(),
                AnalysisMode::Static,
                vec![],
            )
            .unwrap();

            let payload = build_ast_explorer_payload(&ctx);
            let cst = payload.get("cst").unwrap();
            assert_eq!(cst.get("kind").and_then(|v| v.as_str()), Some("program"));
            let children = cst.get("children").and_then(|v| v.as_array()).unwrap();
            assert!(!children.is_empty());
            let assign_node = &children[0];
            assert_eq!(
                assign_node.get("kind").and_then(|v| v.as_str()),
                Some("binary_operator")
            );

            let ir = payload.get("ir").unwrap();
            assert_eq!(ir.get("kind").and_then(|v| v.as_str()), Some("Program"));
            let ir_stmts = ir.get("children").and_then(|v| v.as_array()).unwrap();
            assert_eq!(ir_stmts.len(), 1);
            assert_eq!(
                ir_stmts[0].get("kind").and_then(|v| v.as_str()),
                Some("AssignmentStmt")
            );
        }

        #[test]
        fn test_ast_scenario_2_nested_arithmetic() {
            let code = "df$total <- df$price * df$quantity\n";
            let ctx = AnalysisContext::run_from_source(
                PathBuf::from("test.R"),
                code.to_string(),
                AnalysisMode::Static,
                vec![],
            )
            .unwrap();

            let payload = build_ast_explorer_payload(&ctx);
            let ir = payload.get("ir").unwrap();
            let ir_stmts = ir.get("children").and_then(|v| v.as_array()).unwrap();
            assert_eq!(ir_stmts.len(), 1);
            let assign = &ir_stmts[0];
            let assign_children = assign.get("children").and_then(|v| v.as_array()).unwrap();
            assert_eq!(assign_children.len(), 2);
            assert_eq!(
                assign_children[0].get("kind").and_then(|v| v.as_str()),
                Some("LValue::Extraction")
            );
            assert_eq!(
                assign_children[1].get("kind").and_then(|v| v.as_str()),
                Some("BinaryOpExpr")
            );
            assert_eq!(
                assign_children[1].get("detail").and_then(|v| v.as_str()),
                Some("Mul")
            );
        }

        #[test]
        fn test_ast_scenario_3_function_call() {
            let code = "result <- sum(c(1, 2, 3))\n";
            let ctx = AnalysisContext::run_from_source(
                PathBuf::from("test.R"),
                code.to_string(),
                AnalysisMode::Static,
                vec![],
            )
            .unwrap();

            let payload = build_ast_explorer_payload(&ctx);
            let ir = payload.get("ir").unwrap();
            let ir_stmts = ir.get("children").and_then(|v| v.as_array()).unwrap();
            let assign = &ir_stmts[0];
            let assign_children = assign.get("children").and_then(|v| v.as_array()).unwrap();
            let val_call = &assign_children[1];
            assert_eq!(
                val_call.get("kind").and_then(|v| v.as_str()),
                Some("CallExpr")
            );
        }

        #[test]
        fn test_ast_scenario_4_nested_calls_and_operators() {
            let code = "result <- mean(df$price + df$tax)\n";
            let ctx = AnalysisContext::run_from_source(
                PathBuf::from("test.R"),
                code.to_string(),
                AnalysisMode::Static,
                vec![],
            )
            .unwrap();

            let payload = build_ast_explorer_payload(&ctx);
            let ir = payload.get("ir").unwrap();
            let ir_stmts = ir.get("children").and_then(|v| v.as_array()).unwrap();
            let assign = &ir_stmts[0];
            let assign_children = assign.get("children").and_then(|v| v.as_array()).unwrap();
            let call = &assign_children[1];
            assert_eq!(call.get("kind").and_then(|v| v.as_str()), Some("CallExpr"));
            let call_children = call.get("children").and_then(|v| v.as_array()).unwrap();
            assert_eq!(
                call_children[0].get("kind").and_then(|v| v.as_str()),
                Some("IdentifierExpr")
            );
            assert_eq!(
                call_children[1].get("kind").and_then(|v| v.as_str()),
                Some("PositionalArg")
            );
        }

        #[test]
        fn test_ast_scenario_5_multiple_statements() {
            let code = "x <- 10\ny <- x * 2\nprint(y)\n";
            let ctx = AnalysisContext::run_from_source(
                PathBuf::from("test.R"),
                code.to_string(),
                AnalysisMode::Static,
                vec![],
            )
            .unwrap();

            let payload = build_ast_explorer_payload(&ctx);
            let ir = payload.get("ir").unwrap();
            let ir_stmts = ir.get("children").and_then(|v| v.as_array()).unwrap();
            assert_eq!(ir_stmts.len(), 3);
            assert_eq!(
                ir_stmts[0].get("kind").and_then(|v| v.as_str()),
                Some("AssignmentStmt")
            );
            assert_eq!(
                ir_stmts[1].get("kind").and_then(|v| v.as_str()),
                Some("AssignmentStmt")
            );
            assert_eq!(
                ir_stmts[2].get("kind").and_then(|v| v.as_str()),
                Some("ExprStmt")
            );

            let summary = payload.get("summary").unwrap();
            assert!(
                summary
                    .get("ir_node_count")
                    .and_then(|v| v.as_f64())
                    .unwrap()
                    > 3.0
            );
        }

        #[test]
        fn test_ast_scenario_6_invalid_or_incomplete_syntax() {
            let code = "df$total <- df$price *\n";
            let (status, resp_json) = handle_api_analyze(&format!(
                r#"{{"source":"{}","file_name":"broken.R"}}"#,
                code.replace('\n', "\\n")
            ));
            assert_eq!(status, 200);
            let parsed = JsonValue::parse(&resp_json).unwrap();
            let ast_payload = parsed.get("ast_explorer").unwrap();
            let summary = ast_payload.get("summary").unwrap();
            assert_eq!(
                summary.get("has_syntax_errors").and_then(|v| v.as_bool()),
                Some(true)
            );
            assert!(
                summary
                    .get("error_node_count")
                    .and_then(|v| v.as_f64())
                    .unwrap()
                    > 0.0
            );
        }

        #[test]
        fn test_ast_scenario_7_empty_source() {
            let (status, resp_json) = handle_api_analyze(r#"{"source":"","file_name":"empty.R"}"#);
            assert_eq!(status, 200);
            let parsed = JsonValue::parse(&resp_json).unwrap();
            let ast_payload = parsed.get("ast_explorer").unwrap();
            let summary = ast_payload.get("summary").unwrap();
            assert_eq!(
                summary.get("cst_node_count").and_then(|v| v.as_f64()),
                Some(0.0)
            );
            assert_eq!(
                summary.get("ir_node_count").and_then(|v| v.as_f64()),
                Some(0.0)
            );
        }

        #[test]
        fn test_ast_scenario_8_source_span_correctness() {
            let code = "x <- 10";
            let ctx = AnalysisContext::run_from_source(
                PathBuf::from("test.R"),
                code.to_string(),
                AnalysisMode::Static,
                vec![],
            )
            .unwrap();

            let payload = build_ast_explorer_payload(&ctx);
            let cst = payload.get("cst").unwrap();
            let span = cst.get("span").unwrap();
            assert_eq!(span.get("start_line").and_then(|v| v.as_u64()), Some(1));
            assert_eq!(span.get("start_col").and_then(|v| v.as_u64()), Some(0));
            assert_eq!(span.get("end_line").and_then(|v| v.as_u64()), Some(1));
            assert_eq!(span.get("end_col").and_then(|v| v.as_u64()), Some(7));
        }

        #[test]
        fn test_ast_scenario_9_live_integration_source_reanalysis() {
            let req1 = r#"{"source":"x <- 10","revision":1}"#;
            let (status1, resp1) = handle_api_analyze(req1);
            assert_eq!(status1, 200);
            let parsed1 = JsonValue::parse(&resp1).unwrap();
            let count1 = parsed1
                .get("ast_explorer")
                .and_then(|a| a.get("summary"))
                .and_then(|s| s.get("cst_node_count"))
                .and_then(|v| v.as_f64())
                .unwrap();

            let req2 = r#"{"source":"x <- 20 + y * 3","revision":2}"#;
            let (status2, resp2) = handle_api_analyze(req2);
            assert_eq!(status2, 200);
            let parsed2 = JsonValue::parse(&resp2).unwrap();
            let count2 = parsed2
                .get("ast_explorer")
                .and_then(|a| a.get("summary"))
                .and_then(|s| s.get("cst_node_count"))
                .and_then(|v| v.as_f64())
                .unwrap();

            assert_ne!(count1, count2);
            assert!(count2 > count1);
            assert_eq!(parsed2.get("revision").and_then(|v| v.as_i64()), Some(2));
        }

        #[test]
        fn test_ast_scenario_10_regression_all_existing_views_work() {
            let showcase_source = include_str!("../../../fixtures/showcase_pipeline.R");
            let (status, resp_json) = handle_api_analyze(&format!(
                r#"{{"source":"{}","file_name":"showcase_pipeline.R"}}"#,
                showcase_source
                    .replace('\\', "\\\\")
                    .replace('\n', "\\n")
                    .replace('"', "\\\"")
            ));
            assert_eq!(status, 200);
            let parsed = JsonValue::parse(&resp_json).unwrap();

            let prov = parsed.get("provenance").unwrap();
            assert!(!prov
                .get("nodes")
                .and_then(|v| v.as_array())
                .unwrap()
                .is_empty());
            assert!(!prov
                .get("edges")
                .and_then(|v| v.as_array())
                .unwrap()
                .is_empty());

            let pdg = parsed.get("pdg").unwrap();
            assert!(!pdg
                .get("nodes")
                .and_then(|v| v.as_array())
                .unwrap()
                .is_empty());
            assert!(!pdg
                .get("edges")
                .and_then(|v| v.as_array())
                .unwrap()
                .is_empty());

            let slices = parsed.get("slices").unwrap();
            assert!(!slices.as_object().unwrap().is_empty());

            let schema = parsed.get("schema_evolution").unwrap();
            assert!(!schema.as_array().unwrap().is_empty());

            let ast = parsed.get("ast_explorer").unwrap();
            assert!(ast.get("cst").is_some());
            assert!(ast.get("ir").is_some());
            let summary = ast.get("summary").unwrap();
            assert!(
                summary
                    .get("cst_node_count")
                    .and_then(|v| v.as_f64())
                    .unwrap()
                    > 0.0
            );
            assert!(
                summary
                    .get("ir_node_count")
                    .and_then(|v| v.as_f64())
                    .unwrap()
                    > 0.0
            );
        }
    }

    mod studio_redesign_tests {
        use crate::format_html::build_default_studio_html;

        fn assert_studio_view_shortcuts(html: &str) {
            let shortcuts = [
                ("1", "view-ast", "tab-ast", "ast", "Switch to AST Explorer"),
                ("2", "view-pdg", "tab-pdg", "pdg", "Switch to PDG Graph"),
                (
                    "3",
                    "view-schema",
                    "tab-schema",
                    "schema",
                    "Switch to Schema Evolution",
                ),
                (
                    "4",
                    "view-prov",
                    "tab-provenance",
                    "provenance",
                    "Switch to Provenance",
                ),
                (
                    "5",
                    "view-slice",
                    "tab-slice",
                    "slice",
                    "Switch to Backward Slice",
                ),
            ];

            let mut previous = 0usize;
            for (key, action, tab, view, label) in shortcuts {
                let marker = format!(r#"data-action="{action}""#);
                let at = html
                    .find(&marker)
                    .unwrap_or_else(|| panic!("palette missing {action}"));
                assert!(
                    at >= previous,
                    "palette order must be AST, PDG, Schema, Provenance, Slice"
                );
                previous = at;

                let end = html[at..]
                    .find("</div>")
                    .map(|offset| at + offset)
                    .unwrap_or(html.len());
                let item = &html[at..end];
                assert!(
                    item.contains(label),
                    "palette label for {action} must be {label}"
                );
                assert!(
                    item.contains(&format!(r#"<kbd class="kbd-shortcut">{key}</kbd>"#)),
                    "palette shortcut for {label} must be {key}"
                );

                let key_marker = format!("e.key === '{key}'");
                let key_at = html
                    .find(&key_marker)
                    .unwrap_or_else(|| panic!("missing key handler for {key}"));
                let handler_window = &html[key_at..(key_at + 80).min(html.len())];
                assert!(
                    handler_window.contains(&format!("switchView('{view}')")),
                    "key {key} must open {view}"
                );

                let click =
                    format!("action === '{action}') document.getElementById('{tab}')?.click();");
                assert!(
                    html.contains(&click),
                    "palette action {action} must open {tab}"
                );
            }
        }

        #[test]
        fn test_studio_premium_header_markup_and_css() {
            let html = build_default_studio_html();

            // Row 1 Identity
            assert!(html.contains(r#"<img class="brand-mark" src="/brand/rparse-mark-96.png""#));
            assert!(html.contains(r#"class="workspace-inspector collapsed""#));
            assert!(html.contains(r#"id="btn-theme-toggle""#));
            assert!(html.contains(r#"id="btn-pdg-neighborhood""#));
            assert!(html.contains("function choosePdgOrientation"));
            assert!(html.contains("function placePdgViewport"));
            assert!(html.contains("This is one hop, not every transitive dependency."));
            assert!(html.contains(r#"class="palette-item-icon""#));
            assert!(html.contains(r#"class="palette-search-icon""#));
            assert!(!html.contains("📝 Load Program"));
            assert!(!html.contains("📊 Switch to Column Provenance"));
            assert!(!html.contains("🕸️ Switch to Unified PDG Graph"));
            assert!(!html.contains("🎯 Switch to Backward Slice"));
            assert!(!html.contains("🧬 Switch to Schema Evolution"));
            assert!(!html.contains("🌲 Switch to AST"));
            assert!(!html.contains("⚡ Reanalyze Program"));
            assert!(!html.contains("↩️ Toggle Source Word Wrap"));
            assert!(!html.contains("📋 Copy Current CLI Command"));
            assert!(!html.contains("📖 Open How to Use Guide"));
            assert!(html.contains(r#"class="ui-icon""#));
            assert!(!html.contains("🔍"));
            assert!(!html.contains("🎯"));
            assert!(!html.contains("📖"));
            assert!(!html.contains("⚡"));
            assert!(!html.contains("⚠️"));
            assert!(html.contains("Jump to Source (Line"));
            assert!(html.contains("Compute Backward Slice for"));
            assert!(html.contains("Filter Dependency Applied"));
            assert!(html.contains(">Guide</span>"));
            assert!(html.contains(r#"id="btn-forced-black""#));
            assert!(html.contains(r#"html[data-theme="dark"]"#));
            assert!(html.contains(r#"html[data-theme="black"]"#));
            assert!(html.contains("rparse-studio-theme"));
            assert!(html.contains(r#"<span class="brand-logo">R—PARSE</span>"#));
            assert!(!html.contains(r#"<span class="header-sub">Studio</span>"#));
            assert!(!html.contains(r#"<span class="header-sep" aria-hidden="true"></span>"#));
            assert!(!html.contains(r#"id="header-file-name""#));

            // Row 1 Metrics (status badge and search shortcut hint removed)
            assert!(!html.contains(r#"id="analysis-status-badge""#));
            assert!(!html.contains(r#"id="header-status-dot""#));
            assert!(!html.contains(r#"<kbd class="kbd-shortcut">⌘K</kbd>"#));
            assert!(html.contains(r#"id="header-mode""#));
            assert!(html.contains(r#"id="stat-unk""#));
            assert!(html.contains(r#"id="stat-stmts""#));
            assert!(html.contains(r#"id="stat-cols""#));
            assert!(html.contains(r#"id="stat-nodes""#));

            // Row 1 Actions (Evidence button removed per specification)
            assert!(!html.contains(r#"id="btn-toggle-evidence""#));
            assert!(html.contains(r#"id="btn-new-analysis""#));
            assert!(html.contains(r#""source_code":"""#));
            assert!(html.contains(r#""statement_count":0"#));
            assert!(html.contains(r#"id="load-program-modal""#));
            assert!(html.contains(".view-toolbar"));
            assert!(html.contains("min-height: 38px"));
            assert!(html.contains("flex-wrap: wrap"));
            assert!(html.contains(".toolbar-actions"));
            assert!(html.contains("min-height: 40px"));
            assert!(html.contains(r#"html[data-theme="dark"] .select-dropdown"#));
            assert!(html.contains(r#"html[data-theme="black"] .select-dropdown"#));
            assert!(html.contains(r#"stroke='%23A8A296'"#));
            assert!(html.contains(r#"stroke='%23B7B3AA'"#));
            assert!(html.contains(r#"class="btn btn-secondary" id="btn-ast-expand-all""#));
            assert!(html.contains(r#"class="btn btn-secondary" id="btn-ast-collapse-all""#));
            assert!(!html.contains("btn-outline"));
            assert!(html.contains(r#"<span class="toolbar-label">Legend:</span>"#));
            assert!(!html.contains("text-transform:uppercase; letter-spacing:0.04em;"));

            // Row 2 Analysis Navigation (workflow order: AST → PDG → Schema → Provenance → Slice)
            let tab_ast = html.find(r#"id="tab-ast""#).expect("AST tab");
            let tab_pdg = html.find(r#"id="tab-pdg""#).expect("PDG tab");
            let tab_schema = html.find(r#"id="tab-schema""#).expect("Schema tab");
            let tab_provenance = html.find(r#"id="tab-provenance""#).expect("Provenance tab");
            let tab_slice = html.find(r#"id="tab-slice""#).expect("Slice tab");
            assert!(
                tab_ast < tab_pdg
                    && tab_pdg < tab_schema
                    && tab_schema < tab_provenance
                    && tab_provenance < tab_slice,
                "nav tabs must follow AST → PDG → Schema → Provenance → Slice"
            );
            assert_studio_view_shortcuts(&html);

            // Ensure no rectangular boxed tab borders or pill containers
            assert!(html.contains(".nav-tab.active"));
            assert!(html.contains("border-bottom: 2px solid var(--accent-blue)"));
        }

        #[test]
        fn test_studio_source_editor_word_wrap_features() {
            let html = build_default_studio_html();

            // Word wrap button & accessible states
            assert!(html.contains(r#"id="btn-toggle-wrap""#));
            assert!(html.contains("Word wrap"));
            assert!(html.contains(r#"aria-label="Toggle word wrap""#));
            assert!(html.contains(r#"aria-pressed="true""#));

            // Default state has word-wrap-enabled
            assert!(html.contains(r#"class="pane-source word-wrap-enabled" id="source-pane""#));
            assert!(html.contains(r#"id="btn-toggle-source""#));
            assert!(html.contains(r#"class="source-rail-chevron""#));
            assert!(!html.contains(r#"id="scenario-summary-card""#));
            assert!(!html.contains(r#"id="btn-toggle-scenario""#));

            // CSS rules for word wrap
            assert!(html.contains("#source-pane.word-wrap-enabled .source-viewer"));
            assert!(html.contains("#source-pane.word-wrap-enabled .line-text"));
            assert!(html.contains("white-space: pre-wrap"));
            assert!(html.contains("overflow-wrap: anywhere"));

            // CSS rules for unwrapped state
            assert!(html.contains("#source-pane:not(.word-wrap-enabled) .source-viewer"));
            assert!(html.contains("#source-pane:not(.word-wrap-enabled) .line-text"));
            assert!(html.contains("white-space: pre"));

            // Client JS word wrap logic
            assert!(html.contains("isWordWrapEnabled = true"));
            assert!(html.contains("applyWordWrapState()"));
        }

        #[test]
        fn test_how_to_use_documentation_integration() {
            let html = build_default_studio_html();
            assert!(html.contains(r#"href="/how-to-use""#));
            assert!(html.contains(r#"id="link-how-to-use""#));

            // Verify documentation file is accessible and non-empty
            let doc_content = include_str!("../../../../how-to-use.html");
            assert!(doc_content.contains("How to Use R-PARSE"));
            assert!(doc_content.contains("Welcome to R-PARSE"));
            assert!(doc_content.contains("Visual Analysis Studio"));
            assert!(doc_content.contains("Command Line Interface"));
            assert!(doc_content.contains("Column Provenance Explorer"));
            assert!(doc_content.contains("Unified Program Dependence Graph"));
            assert!(doc_content.contains("Backward Program Slice"));
            assert!(doc_content.contains("Dataframe Schema Evolution"));
            assert!(doc_content.contains("AST &amp; CST Syntax Explorer"));
            assert!(doc_content.contains("Common Errors &amp; Solutions"));
        }
    }
}
