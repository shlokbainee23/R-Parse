//! Source file metadata, identification, and line-index utilities.
//!
//! Provides [`SourceId`] and [`SourceFile`] to identify source code units
//! and compute accurate source positions.

use crate::span::{Position, Span};
use std::fmt;
use std::path::{Path, PathBuf};

/// Unique identifier for a source code file or snippet.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SourceId(u32);

impl SourceId {
    /// Creates a new `SourceId` with the given numeric ID.
    #[inline]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the raw integer value of the identifier.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns the index as a `usize`.
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Debug for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SourceId({})", self.0)
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "src#{}", self.0)
    }
}

/// Represents an in-memory source document with line indexing.
#[derive(Clone, PartialEq, Eq)]
pub struct SourceFile {
    id: SourceId,
    name: String,
    path: Option<PathBuf>,
    content: String,
    line_starts: Vec<u32>,
}

impl SourceFile {
    /// Creates a new `SourceFile` from a name and content string.
    pub fn new(id: SourceId, name: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        let line_starts = compute_line_starts(&content);
        Self {
            id,
            name: name.into(),
            path: None,
            content,
            line_starts,
        }
    }

    /// Creates a new `SourceFile` associated with a filesystem path.
    pub fn with_path(
        id: SourceId,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        content: impl Into<String>,
    ) -> Self {
        let content = content.into();
        let line_starts = compute_line_starts(&content);
        Self {
            id,
            name: name.into(),
            path: Some(path.into()),
            content,
            line_starts,
        }
    }

    /// Returns the unique [`SourceId`].
    #[inline]
    pub fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the display name or label of the source.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the filesystem path, if any.
    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Returns the full UTF-8 source string.
    #[inline]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the byte length of the content.
    #[inline]
    pub fn byte_len(&self) -> usize {
        self.content.len()
    }

    /// Returns the number of lines in this source file.
    #[inline]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Converts a 0-based byte offset to a [`Position`] (1-based line, 0-based column).
    pub fn position_at_byte(&self, byte_offset: u32) -> Position {
        let offset = byte_offset.min(self.content.len() as u32);
        // Binary search for the line start
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };

        let line_start = self.line_starts[line_idx];
        let col = offset.saturating_sub(line_start);

        Position {
            line: (line_idx as u32) + 1,
            column: col,
        }
    }

    /// Builds a [`Span`] from start and end byte offsets.
    pub fn span_for_bytes(&self, start_byte: u32, end_byte: u32) -> Span {
        let start_pos = self.position_at_byte(start_byte);
        let end_pos = self.position_at_byte(end_byte);
        Span::new(
            start_byte,
            end_byte,
            start_pos.line,
            start_pos.column,
            end_pos.line,
            end_pos.column,
        )
    }

    /// Extracts the source text for the given [`Span`].
    pub fn slice(&self, span: &Span) -> Option<&str> {
        span.slice(&self.content)
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceFile")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("path", &self.path)
            .field("bytes", &self.content.len())
            .field("lines", &self.line_starts.len())
            .finish()
    }
}

fn compute_line_starts(content: &str) -> Vec<u32> {
    let mut starts = vec![0];
    for (idx, byte) in content.bytes().enumerate() {
        if byte == b'\n' {
            starts.push((idx + 1) as u32);
        }
    }
    starts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_file_positions() {
        let code = "x <- 10\ny <- 20\nz <- x + y\n";
        let sf = SourceFile::new(SourceId::new(1), "test.R", code);

        assert_eq!(sf.line_count(), 4); // lines 1, 2, 3, and trailing empty line 4
        assert_eq!(sf.position_at_byte(0), Position::new(1, 0));
        assert_eq!(sf.position_at_byte(2), Position::new(1, 2));
        assert_eq!(sf.position_at_byte(8), Position::new(2, 0)); // 'y' on line 2
    }

    #[test]
    fn test_source_file_span_creation() {
        let code = "foo(bar)\n";
        let sf = SourceFile::new(SourceId::new(0), "test.R", code);
        let span = sf.span_for_bytes(4, 7);
        assert_eq!(span.start_byte, 4);
        assert_eq!(span.end_byte, 7);
        assert_eq!(span.start_line, 1);
        assert_eq!(span.start_col, 4);
        assert_eq!(span.end_line, 1);
        assert_eq!(span.end_col, 7);
        assert_eq!(sf.slice(&span), Some("bar"));
    }
}
