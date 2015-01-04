pub mod semantic;
pub mod eval;

#[deriving(Show)]
pub struct Program(pub Statement);

#[deriving(Show)]
pub enum Statement {
    Block(Vec<Statement>),
    Declare(Type, String, Expression),
    Assign(String, Expression),
    Print(Expression)
}

#[deriving(Show, Clone)]
pub enum Expression {
    Int(i32),
    Char(char),
    Bool(bool),
    Identifier(String),
    Unary(UnaryOp, Box<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>)
}

#[deriving(Show, Clone)]
pub enum UnaryOp {
    Neg
}

#[deriving(Show, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul
}

#[deriving(Show, Clone)]
pub enum Type {
    Unknown,
    Error,
    Any,
    Int,
    Char,
    Bool,
    Array(Box<Type>),
    Pair(Box<Type>, Box<Type>)
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match *self {
            Type::Unknown => match *other { Type::Unknown => true, _ => false },
            Type::Error => false,
            Type::Any => true,
            Type::Int => match *other { Type::Int => true, _ => false },
            Type::Char => match *other { Type::Char => true, _ => false },
            Type::Bool => match *other { Type::Bool => true, _ => false },
            _ => false
        }
    }
}
