//! Source spans and location representation for R-PARSE.
//!
//! # Indexing Conventions
//! - **Byte Offsets (`start_byte`, `end_byte`):** 0-based byte indices into UTF-8 source strings.
//!   The range is half-open: `[start_byte, end_byte)`.
//! - **Line Numbers (`start_line`, `end_line`):** 1-based line numbers. Line 1 is the first line of the file.
//! - **Column Numbers (`start_col`, `end_col`):** 0-based column offsets (UTF-8 byte/character column within line).
//!
//! These conventions match standard compiler diagnostics and Tree-sitter's point/byte representation
//! (where `row` is converted from 0-based to 1-based line).

use std::fmt;

/// A position in source code represented by a 1-based line and 0-based column.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
    /// 1-based line number (line 1 is the first line).
    pub line: u32,
    /// 0-based column offset.
    pub column: u32,
}

impl Position {
    /// Creates a new `Position`.
    #[inline]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A physical span in source code.
///
/// Contains byte range offsets as well as 1-based line and 0-based column boundaries.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Span {
    /// 0-based start byte offset (inclusive).
    pub start_byte: u32,
    /// 0-based end byte offset (exclusive).
    pub end_byte: u32,
    /// 1-based start line number.
    pub start_line: u32,
    /// 0-based start column offset.
    pub start_col: u32,
    /// 1-based end line number.
    pub end_line: u32,
    /// 0-based end column offset.
    pub end_col: u32,
}

impl Span {
    /// Creates a new `Span` with explicit byte and line/column coordinates.
    pub const fn new(
        start_byte: u32,
        end_byte: u32,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            start_byte,
            end_byte,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    /// Creates an empty span at line 1, column 0, byte 0.
    pub const fn empty() -> Self {
        Self {
            start_byte: 0,
            end_byte: 0,
            start_line: 1,
            start_col: 0,
            end_line: 1,
            end_col: 0,
        }
    }

    /// Returns the start position as a [`Position`].
    #[inline]
    pub const fn start_pos(&self) -> Position {
        Position {
            line: self.start_line,
            column: self.start_col,
        }
    }

    /// Returns the end position as a [`Position`].
    #[inline]
    pub const fn end_pos(&self) -> Position {
        Position {
            line: self.end_line,
            column: self.end_col,
        }
    }

    /// Returns the length in bytes.
    #[inline]
    pub const fn byte_len(&self) -> usize {
        (self.end_byte.saturating_sub(self.start_byte)) as usize
    }

    /// Returns true if the span covers zero bytes.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.start_byte >= self.end_byte
    }

    /// Checks if a 0-based byte offset falls within this span `[start_byte, end_byte)`.
    #[inline]
    pub const fn contains_byte(&self, byte_offset: u32) -> bool {
        byte_offset >= self.start_byte && byte_offset < self.end_byte
    }

    /// Checks if this span completely encloses `other`.
    pub fn contains_span(&self, other: &Span) -> bool {
        self.start_byte <= other.start_byte && self.end_byte >= other.end_byte
    }

    /// Merges two spans into a single enclosing span spanning from the earliest start to the latest end.
    pub fn merge(&self, other: &Span) -> Span {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }

        let (start_byte, start_line, start_col) = if self.start_byte <= other.start_byte {
            (self.start_byte, self.start_line, self.start_col)
        } else {
            (other.start_byte, other.start_line, other.start_col)
        };

        let (end_byte, end_line, end_col) = if self.end_byte >= other.end_byte {
            (self.end_byte, self.end_line, self.end_col)
        } else {
            (other.end_byte, other.end_line, other.end_col)
        };

        Span {
            start_byte,
            end_byte,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    /// Slices source text corresponding to this span.
    ///
    /// Returns `None` if the byte range exceeds the source length or does not align with UTF-8 character boundaries.
    pub fn slice<'a>(&self, source: &'a str) -> Option<&'a str> {
        let start = self.start_byte as usize;
        let end = self.end_byte as usize;
        if end <= source.len() && start <= end {
            source.get(start..end)
        } else {
            None
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Span({}:{}..{}:{}, bytes: {}..{})",
            self.start_line,
            self.start_col,
            self.end_line,
            self.end_col,
            self.start_byte,
            self.end_byte
        )
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}

impl Span {
    /// Serializes this source span into a structured [`JsonValue`].
    pub fn to_json(&self) -> crate::json::JsonValue {
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "start_byte".to_string(),
            crate::json::JsonValue::Number(self.start_byte as f64),
        );
        map.insert(
            "end_byte".to_string(),
            crate::json::JsonValue::Number(self.end_byte as f64),
        );
        map.insert(
            "start_line".to_string(),
            crate::json::JsonValue::Number(self.start_line as f64),
        );
        map.insert(
            "start_col".to_string(),
            crate::json::JsonValue::Number(self.start_col as f64),
        );
        map.insert(
            "end_line".to_string(),
            crate::json::JsonValue::Number(self.end_line as f64),
        );
        map.insert(
            "end_col".to_string(),
            crate::json::JsonValue::Number(self.end_col as f64),
        );
        crate::json::JsonValue::Object(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_creation_and_properties() {
        let span = Span::new(0, 10, 1, 0, 1, 10);
        assert_eq!(span.byte_len(), 10);
        assert!(!span.is_empty());
        assert!(span.contains_byte(0));
        assert!(span.contains_byte(5));
        assert!(!span.contains_byte(10));
        assert_eq!(span.start_pos(), Position::new(1, 0));
        assert_eq!(span.end_pos(), Position::new(1, 10));
    }

    #[test]
    fn test_span_slice() {
        let text = "x <- 10 + 20";
        let span = Span::new(5, 7, 1, 5, 1, 7);
        assert_eq!(span.slice(text), Some("10"));
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(0, 4, 1, 0, 1, 4);
        let span2 = Span::new(8, 12, 1, 8, 1, 12);
        let merged = span1.merge(&span2);
        assert_eq!(merged.start_byte, 0);
        assert_eq!(merged.end_byte, 12);
        assert_eq!(merged.start_line, 1);
        assert_eq!(merged.start_col, 0);
        assert_eq!(merged.end_line, 1);
        assert_eq!(merged.end_col, 12);
    }

    #[test]
    fn test_empty_span() {
        let empty = Span::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.byte_len(), 0);
    }
}
