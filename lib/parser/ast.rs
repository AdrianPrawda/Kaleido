use super::span::Span;

pub type Program = Vec<Stmt>;

#[derive(PartialEq, Debug, Clone)]
pub struct Stmt {
    span: Span,
    kind: StmtKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum StmtKind {
    Return(Box<Expr>),
    Continue,
    Break,
    Assign {
        ident: Identifier,
        value: Box<Expr>,
    },
    While {
        cond: BExpr,
        body: Program,
    },
    If {
        cond: BExpr,
        if_true: Program,
        elif: Option<Vec<Elif>>,
        if_false: Option<Program>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Elif {
    cond: BExpr,
    body: Program,
    span: Span,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Expr {
    span: Span,
    kind: ExprKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExprKind {
    Ident(Identifier),
    Literal(LiteralKind),
    Grouping(Box<ExprKind>),
    BExpr(BExpr),
    AExpr(AExpr),
}

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    String(String),
    Char(char),
    Number(i64),
    Decimal(f64),
    Bool(bool),
}

#[derive(PartialEq, Debug, Clone)]
pub struct AExpr {
    span: Span,
    kind: AExprKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AExprKind {
    Ident(Identifier),
    Grouping(Box<AExpr>),
    Int(i64),
    Decimal(f64),
    Infix {
        left: Box<AExpr>,
        op: AOp,
        right: Box<AExpr>,
    },
    Prefix {
        op: APrefixOp,
        expr: Box<AExpr>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum AOp {
    Plus,
    Minus,
    Div,
    Mult,
    Modulo,
    LShift,
    RShift,
}

#[derive(PartialEq, Debug, Clone)]
pub enum APrefixOp {
    Plus,
    Minus,
}

#[derive(PartialEq, Debug, Clone)]
pub struct BExpr {
    span: Span,
    kind: BExprKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BExprKind {
    Ident(Identifier),
    Grouping(Box<BExpr>),
    Not(Box<BExpr>),
    True,
    False,
    BInfix {
        left: Box<BExpr>,
        op: BOp,
        right: Box<BExpr>,
    },
    AInfix {
        left: Box<AExpr>,
        op: CmpOp,
        right: Box<AExpr>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum BOp {
    And,
    Or,
    XOr,
}

#[derive(PartialEq, Debug, Clone)]
pub enum CmpOp {
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Identifier {
    name: String,
    kind: IdentifierKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
    Int64(i64),
    Float64(f64),
    Bool(bool),
    String(String),
    Char(char),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Fixity {
    Left,
    Right,
    None,
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = -10,
    Equals = 0,
    Comparison = 10,
    Sum = 20,
    Product = 30,
    Call = 40,
}