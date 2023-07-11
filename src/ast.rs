// TODO: CREATE INDIRECTION TO MAKE ALL TYPES SIZED

enum Statement {
    Expression(Expression),
    Assignment(Assignment),
    While(While),
}

struct Assignment {
    identifier: String,
    type_identifier: Option<String>,
    expression: Expression,
}

struct While {
    condition: Expression,
    body: Scope,
}

enum Expression {
    OperatorCall(OperatorCall),
    FunctionCall(FunctionCall),
    Variable(Variable),
    Literal(Literal),
    // Condition, // TODO
    // Match, // TODO
}

enum OperatorIdentifier {
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
}

struct OperatorCall {
    operator_identifier: OperatorIdentifier,
    lhs: Expression,
    rhs: Expression,
}

struct FunctionCall {
    identifier: String,
    arguments: Vec<Expression>,
}

struct Variable {
    identifier: String,
}

struct Scope {
    statements: Vec<Statement>,
}

enum Literal {
    Int64(i64),
    Float64(f64),
    String(String),
    Boolean(bool),
}

struct Argument {
    variable: Variable,
    variable_type_identifier: String,
}

struct Function {
    identifier: String,
    arguments: Vec<Argument>,
    body: Scope,
}

struct Operator {
    identifier: String,
    arguments: Vec<Argument>,
    body: Scope,
}

struct Struct {
    identifier: String,
    arguments: Vec<Argument>,
}

struct Enum {
    identifier: String,
    struct_identifiers: Vec<String>,
}

enum Declaration {
    Function(Function),
    Operator(Operator),
    Struct(Struct),
    Enum(Enum),
}

struct Program {
    declarations: Vec<Declaration>,
}
