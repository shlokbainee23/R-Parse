//! Stable internal node identity for R-PARSE syntax and semantic trees.
//!
//! Node identifiers are represented as compact 32-bit unsigned integers,
//! decoupling internal R-PARSE representations from Tree-sitter raw node IDs
//! and enabling dense indexing into arenas or lookup tables.

use std::fmt;

/// A stable, compact internal identifier for a syntax or semantic node.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeId(u32);

impl NodeId {
    /// Creates a new `NodeId` from a raw `u32` value.
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw integer value of the identifier.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns the zero-based index for indexing into collections.
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// A sequential allocator for generating unique [`NodeId`] instances.
#[derive(Debug, Clone, Default)]
pub struct NodeIdGenerator {
    next: u32,
}

impl NodeIdGenerator {
    /// Creates a new generator starting at node index 0.
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    /// Allocates and returns the next unique `NodeId`.
    pub fn next_id(&mut self) -> NodeId {
        let id = NodeId::new(self.next);
        self.next = self
            .next
            .checked_add(1)
            .expect("NodeId overflow: exceeded 4,294,967,295 nodes");
        id
    }

    /// Returns the count of IDs allocated so far.
    pub const fn count(&self) -> usize {
        self.next as usize
    }

    /// Resets the generator back to index 0.
    pub fn reset(&mut self) {
        self.next = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_creation_and_accessors() {
        let id = NodeId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(id.index(), 42);
        assert_eq!(format!("{id}"), "#42");
        assert_eq!(format!("{id:?}"), "NodeId(42)");
    }

    #[test]
    fn test_node_id_generator() {
        let mut gen = NodeIdGenerator::new();
        assert_eq!(gen.count(), 0);

        let id0 = gen.next_id();
        let id1 = gen.next_id();
        let id2 = gen.next_id();

        assert_eq!(id0, NodeId::new(0));
        assert_eq!(id1, NodeId::new(1));
        assert_eq!(id2, NodeId::new(2));
        assert_eq!(gen.count(), 3);

        gen.reset();
        assert_eq!(gen.count(), 0);
        assert_eq!(gen.next_id(), NodeId::new(0));
    }
}
