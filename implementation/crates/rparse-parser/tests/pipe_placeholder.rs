use rparse_parser::RParser;

#[test]
fn test_pipe_placeholder_in_named_argument() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "x |> f(y = _)";
    let cst = parser.parse(code).expect("parse");

    assert!(
        !cst.has_errors(),
        "Native pipe with placeholder '_' must not produce syntax errors: {:?}",
        cst.diagnostics()
    );

    let root = cst.root_node();
    let pipe = root.named_child(0).unwrap();
    assert_eq!(pipe.kind(), "binary_operator");

    let rhs = pipe.child_by_field_name("rhs").unwrap();
    assert_eq!(rhs.kind(), "call");

    let args = rhs.child_by_field_name("arguments").unwrap();
    assert_eq!(args.named_child_count(), 1);

    let arg = args.named_child(0).unwrap();
    assert_eq!(arg.kind(), "argument");

    // The argument has name "y" and value "_"
    let name = arg.child_by_field_name("name").unwrap();
    assert_eq!(name.text(cst.source()), Some("y"));

    let val = arg.child_by_field_name("value").unwrap();
    assert_eq!(val.text(cst.source()), Some("_"));
    assert_eq!(val.kind(), "identifier");
}

#[test]
fn test_pipe_placeholder_in_real_world_call() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "mtcars |> lm(mpg ~ disp, data = _)";
    let cst = parser.parse(code).expect("parse");

    assert!(
        !cst.has_errors(),
        "Real-world pipe placeholder syntax must not produce syntax errors"
    );
}

#[test]
fn test_chained_pipes_with_placeholder() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "df |> filter(a > 1) |> mutate(b = 2) |> summary(data = _)";
    let cst = parser.parse(code).expect("parse");

    assert!(
        !cst.has_errors(),
        "Chained pipes with placeholder must not produce syntax errors"
    );
    assert_eq!(cst.diagnostics().len(), 0);
}
