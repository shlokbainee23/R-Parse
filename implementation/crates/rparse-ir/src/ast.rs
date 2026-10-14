//! Normalized Semantic IR node hierarchy for R-PARSE.
//!
//! Represents desugared, normalized R programs with trivia removed.
//! Every node carries a unique [`NodeId`] and physical source [`Span`].

use rparse_core::{NodeId, Span};

/// The top-level root node of a normalized Semantic IR program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub id: NodeId,
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// A normalized statement in the Semantic IR.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assignment(AssignmentStmt),
    Expr(ExprStmt),
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    Return(ReturnStmt),
    Block(Block),
}

impl Stmt {
    pub fn id(&self) -> NodeId {
        match self {
            Self::Assignment(s) => s.id,
            Self::Expr(s) => s.id,
            Self::If(s) => s.id,
            Self::For(s) => s.id,
            Self::While(s) => s.id,
            Self::Return(s) => s.id,
            Self::Block(s) => s.id,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Assignment(s) => s.span,
            Self::Expr(s) => s.span,
            Self::If(s) => s.span,
            Self::For(s) => s.span,
            Self::While(s) => s.span,
            Self::Return(s) => s.span,
            Self::Block(s) => s.span,
        }
    }
}

/// The kind of assignment being performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentKind {
    /// Local environment assignment (`<-`, `->`, and top-level `=`).
    Standard,
    /// Super-assignment (`<<-`, `->>`) targeting an enclosing non-local lexical scope.
    Super,
}

/// Normalized assignment statement desugaring `<-`, `->`, `=`, and `<<-`.
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStmt {
    pub id: NodeId,
    pub target: LValue,
    pub value: Box<Expr>,
    pub kind: AssignmentKind,
    pub span: Span,
}

/// A statement wrapping an evaluated expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub id: NodeId,
    pub expr: Box<Expr>,
    pub span: Span,
}

/// An `if` / `else` conditional branching statement.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub id: NodeId,
    pub condition: Box<Expr>,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
    pub span: Span,
}

/// A `for` iteration loop statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub id: NodeId,
    pub iterator: String,
    pub iterator_span: Span,
    pub iterator_id: NodeId,
    pub iterable: Box<Expr>,
    pub body: Block,
    pub span: Span,
}

/// A `while` loop statement.
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub id: NodeId,
    pub condition: Box<Expr>,
    pub body: Block,
    pub span: Span,
}

/// An explicit `return(...)` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub id: NodeId,
    pub value: Option<Box<Expr>>,
    pub span: Span,
}

/// A sequence of statements enclosed in braces `{ ... }` or forming a logical block.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub id: NodeId,
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// The target of an assignment (lvalue).
#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    /// Simple symbol assignment target (e.g. `x <- 10`).
    Symbol {
        id: NodeId,
        name: String,
        span: Span,
    },
    /// Extraction / subset assignment target (e.g. `df$x <- 10`, `df[["col"]] <- 10`).
    Extraction {
        id: NodeId,
        target: Box<Expr>,
        index: Box<Expr>,
        kind: ExtractKind,
        span: Span,
    },
}

impl LValue {
    pub fn id(&self) -> NodeId {
        match self {
            Self::Symbol { id, .. } => *id,
            Self::Extraction { id, .. } => *id,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Symbol { span, .. } => *span,
            Self::Extraction { span, .. } => *span,
        }
    }
}

/// A normalized expression in the Semantic IR.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    Call(CallExpr),
    Extract(ExtractExpr),
    Pipe(PipeExpr),
    Lambda(LambdaExpr),
    FunctionDef(FunctionDefExpr),
    Unknown(UnknownExpr),
}

impl Expr {
    pub fn id(&self) -> NodeId {
        match self {
            Self::Literal(e) => e.id,
            Self::Identifier(e) => e.id,
            Self::BinaryOp(e) => e.id,
            Self::UnaryOp(e) => e.id,
            Self::Call(e) => e.id,
            Self::Extract(e) => e.id,
            Self::Pipe(e) => e.id,
            Self::Lambda(e) => e.id,
            Self::FunctionDef(e) => e.id,
            Self::Unknown(e) => e.id,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Literal(e) => e.span,
            Self::Identifier(e) => e.span,
            Self::BinaryOp(e) => e.span,
            Self::UnaryOp(e) => e.span,
            Self::Call(e) => e.span,
            Self::Extract(e) => e.span,
            Self::Pipe(e) => e.span,
            Self::Lambda(e) => e.span,
            Self::FunctionDef(e) => e.span,
            Self::Unknown(e) => e.span,
        }
    }
}

/// The kind of literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Numeric(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
    Na,
}

/// A literal constant expression.
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub id: NodeId,
    pub kind: LiteralKind,
    pub raw: String,
    pub span: Span,
}

/// A variable or symbol identifier read.
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierExpr {
    pub id: NodeId,
    pub name: String,
    pub span: Span,
}

/// Binary operator kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Colon,
    Special,
}

/// A binary operator expression.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOpExpr {
    pub id: NodeId,
    pub op: BinaryOp,
    pub op_str: String,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub span: Span,
}

/// Unary operator kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Tilde,
    Help,
}

/// A unary operator expression.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOpExpr {
    pub id: NodeId,
    pub op: UnaryOp,
    pub operand: Box<Expr>,
    pub span: Span,
}

/// A function call expression.
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub id: NodeId,
    pub callee: Box<Expr>,
    pub args: Vec<Argument>,
    pub span: Span,
}

/// The syntax kind of an extraction or subsetting expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractKind {
    /// `df$col` or `df@slot`
    Dollar,
    /// `df[i, j]`
    SingleBracket,
    /// `df[[i]]`
    DoubleBracket,
}

/// An extraction or indexing expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractExpr {
    pub id: NodeId,
    pub target: Box<Expr>,
    pub index: Box<Expr>,
    pub kind: ExtractKind,
    pub span: Span,
}

/// The kind of pipe operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeKind {
    /// Native R 4.1+ pipe `|>`
    Native,
    /// Magrittr pipe `%>%`
    Magrittr,
}

/// An explicit pipe expression preserved before conservative lowering.
#[derive(Debug, Clone, PartialEq)]
pub struct PipeExpr {
    pub id: NodeId,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub kind: PipeKind,
    pub span: Span,
}

/// A formal function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub id: NodeId,
    pub name: String,
    pub default_value: Option<Box<Expr>>,
    pub span: Span,
}

/// A full function definition expression `function(...) { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefExpr {
    pub id: NodeId,
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
}

/// A lambda shorthand expression `\(...) ...`.
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaExpr {
    pub id: NodeId,
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
}

/// An unknown or unmodeled expression construct.
#[derive(Debug, Clone, PartialEq)]
pub struct UnknownExpr {
    pub id: NodeId,
    pub reason: String,
    pub raw: Option<String>,
    pub span: Span,
}

/// An argument in a function call.
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Positional(PositionalArg),
    Named(NamedArg),
}

impl Argument {
    pub fn id(&self) -> NodeId {
        match self {
            Self::Positional(a) => a.id,
            Self::Named(a) => a.id,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Positional(a) => a.span,
            Self::Named(a) => a.span,
        }
    }

    pub fn value(&self) -> &Expr {
        match self {
            Self::Positional(a) => &a.value,
            Self::Named(a) => &a.value,
        }
    }
}

/// A positional argument in a call.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionalArg {
    pub id: NodeId,
    pub value: Box<Expr>,
    pub span: Span,
}

/// A named argument in a call (`name = value`).
#[derive(Debug, Clone, PartialEq)]
pub struct NamedArg {
    pub id: NodeId,
    pub name: String,
    pub name_span: Span,
    pub value: Box<Expr>,
    pub span: Span,
}
