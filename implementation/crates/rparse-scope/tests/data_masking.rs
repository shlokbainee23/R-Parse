use rparse_ir::*;
use rparse_scope::*;

fn analyze(source: &str) -> ScopeModel {
    let ir = parse_source(source).expect("parsing and normalization should succeed");
    resolve_scopes(&ir)
}

#[test]
fn test_data_masking_context_mutate() {
    let src = "mutate(df, total = price * quantity)";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    // Should create a child DataMask scope under root
    let mask_scopes = model.find_data_mask_scopes();
    assert_eq!(mask_scopes.len(), 1);

    let mask_scope = mask_scopes[0];
    assert!(mask_scope.is_data_mask());
    assert_eq!(mask_scope.parent, Some(root));
    match &mask_scope.kind {
        ScopeKind::DataMask { verb, .. } => assert_eq!(verb, "mutate"),
        _ => panic!("Expected DataMask scope"),
    }

    // df should be referenced in root scope
    let df_refs: Vec<_> = model
        .references()
        .iter()
        .filter(|r| r.name == "df")
        .collect();
    assert_eq!(df_refs.len(), 1);
    assert_eq!(df_refs[0].scope_id, root);

    // price and quantity should be referenced as DataMaskBare inside mask_scope
    let price_refs: Vec<_> = model
        .references()
        .iter()
        .filter(|r| r.name == "price")
        .collect();
    assert_eq!(price_refs.len(), 1);
    assert_eq!(price_refs[0].scope_id, mask_scope.id);
    assert_eq!(price_refs[0].kind, RefKind::DataMaskBare);

    let qty_refs: Vec<_> = model
        .references()
        .iter()
        .filter(|r| r.name == "quantity")
        .collect();
    assert_eq!(qty_refs.len(), 1);
    assert_eq!(qty_refs[0].scope_id, mask_scope.id);
    assert_eq!(qty_refs[0].kind, RefKind::DataMaskBare);

    // total should be recorded as a write in mask_scope, not a global variable def
    assert!(!root_scope.symbols.contains_key("total"));
}

#[test]
fn test_explicit_data_reference() {
    let src = "x <- .data$price";
    let model = analyze(src);

    let explicit_refs: Vec<_> = model
        .references()
        .iter()
        .filter(|r| matches!(r.kind, RefKind::DataMaskExplicitData { .. }))
        .collect();

    assert_eq!(explicit_refs.len(), 1);
    match &explicit_refs[0].kind {
        RefKind::DataMaskExplicitData { column } => assert_eq!(column, "price"),
        _ => panic!("Expected DataMaskExplicitData"),
    }
}

#[test]
fn test_explicit_data_bracket_reference() {
    let src = "x <- .data[[\"col_name\"]]";
    let model = analyze(src);

    let explicit_refs: Vec<_> = model
        .references()
        .iter()
        .filter(|r| matches!(r.kind, RefKind::DataMaskExplicitData { .. }))
        .collect();

    assert_eq!(explicit_refs.len(), 1);
    match &explicit_refs[0].kind {
        RefKind::DataMaskExplicitData { column } => assert_eq!(column, "col_name"),
        _ => panic!("Expected DataMaskExplicitData"),
    }
}

#[test]
fn test_explicit_env_reference() {
    let src = "\
multiplier <- 1.5
df |> mutate(adjusted = price * .env$multiplier)
";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    let mult_sym_id = root_scope.symbols["multiplier"];

    let env_refs: Vec<_> = model
        .references()
        .iter()
        .filter(|r| matches!(r.kind, RefKind::DataMaskExplicitEnv { .. }))
        .collect();

    assert_eq!(env_refs.len(), 1);
    match &env_refs[0].kind {
        RefKind::DataMaskExplicitEnv { variable } => assert_eq!(variable, "multiplier"),
        _ => panic!("Expected DataMaskExplicitEnv"),
    }

    // .env$multiplier resolves lexically to the outer variable multiplier!
    assert_eq!(env_refs[0].symbol_id, Some(mult_sym_id));
}

#[test]
fn test_data_masking_verbs_frozen_mvp() {
    let src = "\
df |> filter(status == \"active\")
df |> select(id, name)
df |> rename(new_col = old_col)
df |> transmute(total = price * 2)
";
    let model = analyze(src);
    let mask_scopes = model.find_data_mask_scopes();
    assert_eq!(mask_scopes.len(), 4);

    let verbs: Vec<_> = mask_scopes
        .iter()
        .map(|s| match &s.kind {
            ScopeKind::DataMask { verb, .. } => verb.as_str(),
            _ => "",
        })
        .collect();

    assert_eq!(verbs, vec!["filter", "select", "rename", "transmute"]);
}

#[test]
fn test_non_mvp_verbs_do_not_create_data_mask_scopes() {
    let src = "\
df |> arrange(price)
df |> summarize(m = mean(price))
";
    let model = analyze(src);
    let mask_scopes = model.find_data_mask_scopes();
    assert_eq!(mask_scopes.len(), 0);
}

// =========================================================================
// Scope & Lexical Resolution Tests
// =========================================================================

#[test]
fn test_named_args_not_variable_definitions() {
    let model = analyze("foo(x = 1)");
    let root = model.root_scope();
    let root_scope = model.scope(root);

    // foo should be referenced as call target
    assert!(model
        .references()
        .iter()
        .any(|r| r.name == "foo" && r.kind == RefKind::CallTarget));

    // x must NOT be created as a variable definition or symbol
    assert!(!root_scope.symbols.contains_key("x"));
    assert!(!model.definitions().iter().any(|d| d.name == "x"));
}

#[test]
fn test_super_assignment() {
    let model = analyze("x <<- 1");
    let root = model.root_scope();
    let root_scope = model.scope(root);

    assert!(root_scope.symbols.contains_key("x"));
    let sym_id = root_scope.symbols["x"];
    let defs = model.get_definitions_for_symbol(sym_id);
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].kind, DefKind::Variable(AssignmentKind::Super));
}

#[test]
fn test_super_assignment_targeting_enclosing_scope() {
    let src = "\
x <- 10
f <- function() {
  x <<- 20
}
";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    let x_sym_id = root_scope.symbols["x"];
    let defs = model.get_definitions_for_symbol(x_sym_id);

    // Both x <- 10 and x <<- 20 target the same symbol x in the global scope
    assert_eq!(defs.len(), 2);
    assert_eq!(defs[0].kind, DefKind::Variable(AssignmentKind::Standard));
    assert_eq!(defs[1].kind, DefKind::Variable(AssignmentKind::Super));
}

#[test]
fn test_function_scopes() {
    let src = "\
f <- function(x) {
  y <- x + 1
}
";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    // Global contains f
    assert!(root_scope.symbols.contains_key("f"));
    assert_eq!(root_scope.children.len(), 1);

    // Function scope
    let fn_scope_id = root_scope.children[0];
    let fn_scope = model.scope(fn_scope_id);
    assert!(fn_scope.is_function());
    assert_eq!(fn_scope.parent, Some(root));

    // Function scope defines parameter x and variable y
    assert!(fn_scope.symbols.contains_key("x"));
    assert!(fn_scope.symbols.contains_key("y"));

    let x_sym = model.symbol(fn_scope.symbols["x"]);
    let x_defs = model.get_definitions_for_symbol(x_sym.id);
    assert_eq!(x_defs.len(), 1);
    assert_eq!(x_defs[0].kind, DefKind::Parameter);

    let y_sym = model.symbol(fn_scope.symbols["y"]);
    let y_defs = model.get_definitions_for_symbol(y_sym.id);
    assert_eq!(y_defs.len(), 1);
    assert_eq!(y_defs[0].kind, DefKind::Variable(AssignmentKind::Standard));

    // Reference x in x + 1 resolves to parameter x
    let x_refs = model.get_references_for_symbol(x_sym.id);
    assert_eq!(x_refs.len(), 1);
    assert_eq!(x_refs[0].kind, RefKind::Read);
}

#[test]
fn test_nested_closures_lexical_resolution() {
    let src = "\
f <- function(x) {
  y <- x + 1
  function(z) {
    y + z
  }
}
";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    let outer_fn_scope_id = root_scope.children[0];
    let outer_fn_scope = model.scope(outer_fn_scope_id);
    assert_eq!(outer_fn_scope.children.len(), 1);

    let inner_closure_scope_id = outer_fn_scope.children[0];
    let inner_closure_scope = model.scope(inner_closure_scope_id);

    // Inner closure defines z locally
    assert!(inner_closure_scope.symbols.contains_key("z"));
    let z_sym = model.symbol(inner_closure_scope.symbols["z"]);
    assert_eq!(z_sym.scope_id, inner_closure_scope_id);

    // Resolution: z resolves locally in inner scope
    let resolved_z = model.resolve_symbol(inner_closure_scope_id, "z");
    assert_eq!(resolved_z, Some(z_sym.id));

    // Resolution: y resolves from enclosing outer function scope
    let y_sym = model.symbol(outer_fn_scope.symbols["y"]);
    let resolved_y = model.resolve_symbol(inner_closure_scope_id, "y");
    assert_eq!(resolved_y, Some(y_sym.id));

    // Check inner references
    let y_refs = model.get_references_for_symbol(y_sym.id);
    assert!(y_refs.iter().any(|r| r.scope_id == inner_closure_scope_id));

    let z_refs = model.get_references_for_symbol(z_sym.id);
    assert!(z_refs.iter().any(|r| r.scope_id == inner_closure_scope_id));
}

#[test]
fn test_loop_iterator_definition() {
    let src = "for (i in 1:10) { print(i) }";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    assert!(root_scope.symbols.contains_key("i"));
    let i_sym = model.symbol(root_scope.symbols["i"]);
    let i_defs = model.get_definitions_for_symbol(i_sym.id);
    assert_eq!(i_defs.len(), 1);
    assert_eq!(i_defs[0].kind, DefKind::LoopIterator);

    let i_refs = model.get_references_for_symbol(i_sym.id);
    assert!(i_refs
        .iter()
        .any(|r| r.name == "i" && r.kind == RefKind::Read));
}

#[test]
fn test_multiple_definitions_for_single_symbol() {
    let src = "x <- 1\nx <- 2\n";
    let model = analyze(src);
    let root = model.root_scope();
    let root_scope = model.scope(root);

    assert_eq!(root_scope.symbols.len(), 1);
    let sym_id = root_scope.symbols["x"];
    let defs = model.get_definitions_for_symbol(sym_id);
    assert_eq!(defs.len(), 2);
    assert_eq!(defs[0].kind, DefKind::Variable(AssignmentKind::Standard));
    assert_eq!(defs[1].kind, DefKind::Variable(AssignmentKind::Standard));
}
