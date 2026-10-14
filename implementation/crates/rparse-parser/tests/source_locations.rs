use rparse_parser::RParser;

#[test]
fn test_single_line_source_spans() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "x <- 10 + 20";
    let cst = parser.parse(code).expect("Parsing failed");

    let root = cst.root_node();
    let assign = root.named_child(0).expect("assignment");
    let span = assign.span();

    assert_eq!(span.start_byte, 0);
    assert_eq!(span.end_byte, 12);
    assert_eq!(span.start_line, 1);
    assert_eq!(span.start_col, 0);
    assert_eq!(span.end_line, 1);
    assert_eq!(span.end_col, 12);
    assert_eq!(assign.text(cst.source()), Some("x <- 10 + 20"));

    let lhs = assign.child_by_field_name("lhs").expect("lhs");
    assert_eq!(lhs.span().start_byte, 0);
    assert_eq!(lhs.span().end_byte, 1);
    assert_eq!(lhs.span().start_line, 1);
    assert_eq!(lhs.span().start_col, 0);
    assert_eq!(lhs.span().end_line, 1);
    assert_eq!(lhs.span().end_col, 1);
    assert_eq!(lhs.text(cst.source()), Some("x"));
}

#[test]
fn test_multiline_source_spans() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "f <- function(x) {\n    x + 1\n}";
    let cst = parser.parse(code).expect("Parsing failed");

    let root = cst.root_node();
    let assign = root.named_child(0).expect("assign");
    let fn_node = assign.child_by_field_name("rhs").expect("fn def");
    assert_eq!(fn_node.kind(), "function_definition");

    let span = fn_node.span();
    assert_eq!(span.start_line, 1);
    assert_eq!(span.start_col, 5);
    assert_eq!(span.end_line, 3);
    assert_eq!(span.end_col, 1);
    assert_eq!(
        fn_node.text(cst.source()),
        Some("function(x) {\n    x + 1\n}")
    );
}

#[test]
fn test_utf8_multibyte_source_spans() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    // 'α' is 2 bytes (CE B1) in UTF-8
    // 'β' is 2 bytes (CE B2) in UTF-8
    // '🚀' is 4 bytes (F0 9F 9A 80) in UTF-8
    let code = "α <- \"🚀 rocket\"\nβ <- 42";
    let cst = parser.parse(code).expect("Parsing failed");

    assert!(!cst.has_errors());
    let root = cst.root_node();
    let first = root.named_child(0).expect("first stmt");
    let second = root.named_child(1).expect("second stmt");

    // First stmt: 'α <- "🚀 rocket"'
    // 'α' is 2 bytes: bytes 0..2
    let alpha = first.child_by_field_name("lhs").expect("alpha");
    assert_eq!(alpha.text(cst.source()), Some("α"));
    assert_eq!(alpha.span().start_byte, 0);
    assert_eq!(alpha.span().end_byte, 2);
    assert_eq!(alpha.span().start_line, 1);
    assert_eq!(alpha.span().start_col, 0);

    // Second stmt starts on line 2
    assert_eq!(second.span().start_line, 2);
    let beta = second.child_by_field_name("lhs").expect("beta");
    assert_eq!(beta.text(cst.source()), Some("β"));
    assert_eq!(beta.span().start_line, 2);
}

#[test]
fn test_comment_and_trivia_positions() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "# Leading comment\nx <- 10\n# Trailing comment\n";
    let cst = parser.parse(code).expect("Parsing failed");

    assert!(!cst.has_errors());
    let root = cst.root_node();

    // First child is comment
    let comment = root.child(0).expect("comment");
    assert_eq!(comment.kind(), "comment");
    assert_eq!(comment.text(cst.source()), Some("# Leading comment"));
    assert_eq!(comment.span().start_line, 1);

    // In tree-sitter-r, comments are named nodes.
    // named_child(0) is the leading comment (line 1), named_child(1) is the assignment (line 2).
    let assign = root.named_child(1).expect("assign");
    assert_eq!(assign.kind(), "binary_operator");
    assert_eq!(assign.span().start_line, 2);
    assert_eq!(assign.text(cst.source()), Some("x <- 10"));
}
