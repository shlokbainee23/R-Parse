use rparse_parser::RParser;

#[test]
fn test_assignment_operators() {
    let mut parser = RParser::new().expect("Failed to initialize parser");

    // Left assignment <-
    let cst = parser.parse("x <- 10").expect("parse");
    assert!(!cst.has_errors());
    let op = cst.root_node().named_child(0).unwrap();
    assert_eq!(op.kind(), "binary_operator");
    assert_eq!(op.child_by_field_name("operator").unwrap().kind(), "<-");

    // Right assignment ->
    let cst = parser.parse("10 -> x").expect("parse");
    assert!(!cst.has_errors());
    let op = cst.root_node().named_child(0).unwrap();
    assert_eq!(op.kind(), "binary_operator");
    assert_eq!(op.child_by_field_name("operator").unwrap().kind(), "->");

    // Super-assignment <<-
    let cst = parser.parse("x <<- 100").expect("parse");
    assert!(!cst.has_errors());
    let op = cst.root_node().named_child(0).unwrap();
    assert_eq!(op.kind(), "binary_operator");
    assert_eq!(op.child_by_field_name("operator").unwrap().kind(), "<<-");

    // Walrus operator :=
    let cst = parser.parse("x := 10").expect("parse");
    assert!(!cst.has_errors());
    let op = cst.root_node().named_child(0).unwrap();
    assert_eq!(op.kind(), "binary_operator");
    assert_eq!(op.child_by_field_name("operator").unwrap().kind(), ":=");
}

#[test]
fn test_function_definition() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = r#"
f <- function(x) {
    x + 1
}
"#;
    let cst = parser.parse(code).expect("parse");
    assert!(!cst.has_errors());

    let assign = cst.root_node().named_child(0).unwrap();
    let fn_def = assign.child_by_field_name("rhs").unwrap();
    assert_eq!(fn_def.kind(), "function_definition");

    let params = fn_def.child_by_field_name("parameters").unwrap();
    assert_eq!(params.kind(), "parameters");

    let body = fn_def.child_by_field_name("body").unwrap();
    assert_eq!(body.kind(), "braced_expression");
}

#[test]
fn test_native_pipe_and_placeholder() {
    let mut parser = RParser::new().expect("Failed to initialize parser");

    // Standard native pipe
    let cst = parser.parse("x |> f()").expect("parse");
    assert!(!cst.has_errors());
    let pipe = cst.root_node().named_child(0).unwrap();
    assert_eq!(pipe.kind(), "binary_operator");
    assert_eq!(pipe.child_by_field_name("operator").unwrap().kind(), "|>");

    // Native pipe with placeholder
    let cst = parser.parse("x |> f(y = _)").expect("parse");
    assert!(!cst.has_errors());
    let pipe = cst.root_node().named_child(0).unwrap();
    assert_eq!(pipe.kind(), "binary_operator");
    let rhs_call = pipe.child_by_field_name("rhs").unwrap();
    assert_eq!(rhs_call.kind(), "call");
}

#[test]
fn test_magrittr_pipe() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let cst = parser.parse("x %>% f()").expect("parse");
    assert!(!cst.has_errors());

    let pipe = cst.root_node().named_child(0).unwrap();
    assert_eq!(pipe.kind(), "binary_operator");
    assert_eq!(
        pipe.child_by_field_name("operator").unwrap().kind(),
        "special"
    );
    assert_eq!(
        pipe.child_by_field_name("operator")
            .unwrap()
            .text(cst.source()),
        Some("%>%")
    );
}

#[test]
fn test_formula() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let cst = parser.parse("y ~ x").expect("parse");
    assert!(!cst.has_errors());

    let formula = cst.root_node().named_child(0).unwrap();
    assert_eq!(formula.kind(), "binary_operator");
    assert_eq!(formula.child_by_field_name("operator").unwrap().kind(), "~");

    // Unary formula
    let cst2 = parser.parse("~ x + z").expect("parse");
    assert!(!cst2.has_errors());
    let unary = cst2.root_node().named_child(0).unwrap();
    assert_eq!(unary.kind(), "unary_operator");
}

#[test]
fn test_lambda_shorthand() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let cst = parser.parse(r#"\(x) x + 1"#).expect("parse");
    assert!(!cst.has_errors());

    let lambda = cst.root_node().named_child(0).unwrap();
    assert_eq!(lambda.kind(), "function_definition");
    let params = lambda.child_by_field_name("parameters").unwrap();
    assert_eq!(params.kind(), "parameters");
}

#[test]
fn test_control_structures() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = r#"
if (x > 0) {
    y <- 1
} else {
    y <- 2
}
"#;
    let cst = parser.parse(code).expect("parse");
    assert!(!cst.has_errors());

    let if_node = cst.root_node().named_child(0).unwrap();
    assert_eq!(if_node.kind(), "if_statement");
    assert!(if_node.child_by_field_name("condition").is_some());
    assert!(if_node.child_by_field_name("consequence").is_some());
    assert!(if_node.child_by_field_name("alternative").is_some());
}

#[test]
fn test_indexing_and_subsetting() {
    let mut parser = RParser::new().expect("Failed to initialize parser");

    // Dollar operator: df$x
    let cst = parser.parse("df$x").expect("parse");
    assert!(!cst.has_errors());
    let extract = cst.root_node().named_child(0).unwrap();
    assert_eq!(extract.kind(), "extract_operator");

    // Double bracket: df[["x"]]
    let cst = parser.parse(r#"df[["x"]]"#).expect("parse");
    assert!(!cst.has_errors());
    let subset2 = cst.root_node().named_child(0).unwrap();
    assert_eq!(subset2.kind(), "subset2");

    // Single bracket matrix subset: df[, "x"]
    let cst = parser.parse(r#"df[, "x"]"#).expect("parse");
    assert!(!cst.has_errors());
    let subset = cst.root_node().named_child(0).unwrap();
    assert_eq!(subset.kind(), "subset");
}

#[test]
fn test_nested_function_calls() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "foo(bar(x, y), z)";
    let cst = parser.parse(code).expect("parse");
    assert!(!cst.has_errors());

    let outer = cst.root_node().named_child(0).unwrap();
    assert_eq!(outer.kind(), "call");
    assert_eq!(
        outer
            .child_by_field_name("function")
            .unwrap()
            .text(cst.source()),
        Some("foo")
    );

    let args = outer.child_by_field_name("arguments").unwrap();
    let first_arg = args.named_child(0).unwrap();
    let inner_call = first_arg.named_child(0).unwrap();
    assert_eq!(inner_call.kind(), "call");
    assert_eq!(
        inner_call
            .child_by_field_name("function")
            .unwrap()
            .text(cst.source()),
        Some("bar")
    );
}

#[test]
fn test_comments_and_whitespace() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = r#"
# Comment line 1
x <- 10

# Comment line 2
y <- 20
"#;
    let cst = parser.parse(code).expect("parse");
    assert!(!cst.has_errors());
    // Comments are named nodes in tree-sitter-r: 2 comments + 2 assignments = 4 named children
    assert_eq!(cst.root_node().named_child_count(), 4);

    let statements: Vec<_> = cst
        .root_node()
        .named_children()
        .filter(|c| c.kind() == "binary_operator")
        .collect();
    assert_eq!(statements.len(), 2);
    assert_eq!(statements[0].text(cst.source()), Some("x <- 10"));
    assert_eq!(statements[1].text(cst.source()), Some("y <- 20"));
}
