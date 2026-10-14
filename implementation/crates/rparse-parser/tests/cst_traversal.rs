use rparse_parser::RParser;

#[test]
fn test_cst_root_and_children_traversal() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "x <- 10; y <- 20";
    let cst = parser.parse(code).expect("Parsing failed");

    assert!(!cst.has_errors());
    let root = cst.root_node();
    assert_eq!(root.kind(), "program");
    assert!(root.is_named());

    // Traverse children
    let children: Vec<_> = root.children().collect();
    assert!(!children.is_empty());

    let named_children: Vec<_> = root.named_children().collect();
    assert_eq!(named_children.len(), 2);
    assert_eq!(named_children[0].kind(), "binary_operator");
    assert_eq!(named_children[1].kind(), "binary_operator");
}

#[test]
fn test_cst_field_access() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "foo(a = 1, b = 2)";
    let cst = parser.parse(code).expect("Parsing failed");

    let root = cst.root_node();
    let call = root.named_child(0).expect("call expected");
    assert_eq!(call.kind(), "call");

    let func = call.child_by_field_name("function").expect("callee");
    assert_eq!(func.kind(), "identifier");
    assert_eq!(func.text(cst.source()), Some("foo"));

    let args = call.child_by_field_name("arguments").expect("arguments");
    assert_eq!(args.kind(), "arguments");
    assert!(args.named_child_count() >= 2);
}

#[test]
fn test_cst_parent_and_sibling_navigation() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "a + b";
    let cst = parser.parse(code).expect("Parsing failed");

    let root = cst.root_node();
    let bin_op = root.named_child(0).expect("binary_operator");
    assert_eq!(bin_op.parent().map(|p| p.kind()), Some("program"));

    let lhs = bin_op.child_by_field_name("lhs").expect("lhs");
    assert_eq!(lhs.text(cst.source()), Some("a"));

    let rhs = bin_op.child_by_field_name("rhs").expect("rhs");
    assert_eq!(rhs.text(cst.source()), Some("b"));
}

#[test]
fn test_cst_descendant_lookup_by_byte_range() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "result <- calculate(value)";
    let cst = parser.parse(code).expect("Parsing failed");

    // Look up "calculate" which is at bytes 10..19
    let node = cst.named_descendant_for_byte_range(10, 19).expect("node");
    assert_eq!(node.kind(), "identifier");
    assert_eq!(node.text(cst.source()), Some("calculate"));
}
