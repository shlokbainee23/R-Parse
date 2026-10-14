//! Conversion utilities between Tree-sitter positions and R-PARSE [`Span`].

use rparse_core::{Position, Span};

/// Converts a Tree-sitter [`tree_sitter::Node`] into an R-PARSE [`Span`].
///
/// # Indexing Alignment
/// - Byte offsets: `start_byte` and `end_byte` are directly 0-based byte indices `[start, end)`.
/// - Lines: Tree-sitter 0-based `row` is converted to R-PARSE 1-based `line` (`row + 1`).
/// - Columns: Tree-sitter 0-based `column` is preserved directly as 0-based `col`.
pub fn span_from_ts_node(node: &tree_sitter::Node<'_>) -> Span {
    let start_point = node.start_position();
    let end_point = node.end_position();

    Span::new(
        node.start_byte() as u32,
        node.end_byte() as u32,
        (start_point.row as u32) + 1,
        start_point.column as u32,
        (end_point.row as u32) + 1,
        end_point.column as u32,
    )
}

/// Converts a Tree-sitter [`tree_sitter::Point`] to an R-PARSE [`Position`].
pub fn position_from_ts_point(point: tree_sitter::Point) -> Position {
    Position::new((point.row as u32) + 1, point.column as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_conversion() {
        let pt = tree_sitter::Point { row: 0, column: 5 };
        let pos = position_from_ts_point(pt);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 5);
    }
}
