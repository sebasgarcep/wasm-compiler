#[derive(Debug, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    Function(Box<Function>),
    Operator(Box<Operator>),
    Struct(Box<Struct>),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub identifier: String,
    pub arguments: Vec<Argument>,
    pub body: Box<Scope>,
}

#[derive(Debug, PartialEq)]
pub struct Operator {
    pub identifier: String,
    pub arguments: Vec<Argument>,
    pub body: Box<Scope>,
}

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub identifier: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    VariableDeclaration(Box<VariableDeclaration>),
    Expression(Box<Expression>),
    While(Box<While>),
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub variable: Box<Variable>,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Box<Expression>,
    pub body: Box<Scope>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    OperatorCall(Box<OperatorCall>),
    FunctionCall(Box<FunctionCall>),
    Variable(Box<Variable>),
    Literal(Box<Literal>),
    // Condition, // TODO
    // Match, // TODO
}

#[derive(Debug, PartialEq)]
pub enum OperatorIdentifier {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponential,
    LeftShift,
    RightShift,
    And,
    Or,
    Xor,
    Assignment,
}

#[derive(Debug, PartialEq)]
pub struct OperatorCall {
    pub operator_identifier: OperatorIdentifier,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub identifier: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct Scope {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int64(i64),
    Float64(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub variable: Box<Variable>,
    pub type_annotation: Box<TypeAnnotation>,
}
