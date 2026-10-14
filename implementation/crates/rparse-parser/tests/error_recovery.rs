use rparse_core::DiagnosticSeverity;
use rparse_parser::RParser;

#[test]
fn test_unclosed_if_statement() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let cst = parser.parse("if (").expect("parse should not panic");

    assert!(
        cst.has_errors(),
        "Malformed 'if (' must report syntax errors"
    );
    let diags = cst.diagnostics();
    assert!(
        !diags.is_empty(),
        "Must emit diagnostics for malformed syntax"
    );
    assert!(diags
        .iter()
        .any(|d| d.severity == DiagnosticSeverity::Error));

    // CST root is still accessible
    let root = cst.root_node();
    assert_eq!(root.kind(), "program");
}

#[test]
fn test_unclosed_function_call() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let cst = parser.parse("foo(").expect("parse should not panic");

    assert!(
        cst.has_errors(),
        "Malformed 'foo(' must report syntax errors"
    );
    let diags = cst.diagnostics();
    assert!(!diags.is_empty());

    let root = cst.root_node();
    assert_eq!(root.kind(), "program");
}

#[test]
fn test_incomplete_assignment() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let cst = parser.parse("x <-").expect("parse should not panic");

    assert!(
        cst.has_errors(),
        "Malformed 'x <-' must report syntax errors"
    );
    let diags = cst.diagnostics();
    assert!(!diags.is_empty());
}

#[test]
fn test_unclosed_delimiters_and_recovery() {
    let mut parser = RParser::new().expect("Failed to initialize parser");
    let code = "f <- function(\nx <- 10";
    let cst = parser.parse(code).expect("parse should not panic");

    assert!(cst.has_errors());
    let root = cst.root_node();
    assert_eq!(root.kind(), "program");

    // Check that error diagnostics contain valid source spans
    let diags = cst.diagnostics();
    assert!(!diags.is_empty());
    for diag in &diags {
        assert!(diag.span.is_some());
        let span = diag.span.unwrap();
        assert!(span.start_line >= 1);
    }
}

#[test]
fn test_completely_empty_and_whitespace_only() {
    let mut parser = RParser::new().expect("Failed to initialize parser");

    let cst1 = parser.parse("").expect("empty string");
    assert!(!cst1.has_errors());
    assert_eq!(cst1.root_node().child_count(), 0);

    let cst2 = parser.parse("   \n\n\t  ").expect("whitespace only");
    assert!(!cst2.has_errors());
}
