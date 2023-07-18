#[derive(Debug, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    Function(Box<Function>),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub identifier: String,
    pub type_annotation: Box<TypeAnnotation>,
    pub arguments: Vec<Argument>,
    pub body: Box<Scope>,
}

#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub variable: Box<Variable>,
    pub type_annotation: Box<TypeAnnotation>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct Scope {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Box<Literal>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int64(i64),
    Float64(f64),
}
