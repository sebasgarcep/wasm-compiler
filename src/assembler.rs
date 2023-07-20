use crate::ast::{Program, Declaration, Function, Scope, Statement, Expression, Literal};

pub struct Assembler {
    output: Vec<String>,
}

impl Assembler {
    pub fn to_wat(program: Box<Program>) -> String {
        let mut assembler = Assembler { output: vec![] };
        assembler.assemble_program(*program);
        assembler.output.join("\n")
    }

    pub fn assemble_program(&mut self, program: Program) {
        self.output.push("(module".to_owned());
        for item in program.declarations {
            self.assemble_declaration(item);
        }
        self.output.push("(export \"main\" (func $main))".to_owned());
        self.output.push(")".to_owned());
    }

    pub fn assemble_declaration(&mut self, declaration: Declaration) {
        match declaration {
            Declaration::Function(function) => {
                self.assemble_function(*function);
            },
        };
    }

    pub fn assemble_function(&mut self, function: Function) {
        self.output.push(format!("(func ${} (result i32)", function.identifier)); // FIXME: Implement arguments and return type
        self.assemble_scope(*function.body);
        self.output.push(")".to_owned());
    }

    pub fn assemble_scope(&mut self, scope: Scope) {
        for item in scope.statements {
            self.assemble_statement(item);
        }
    }

    pub fn assemble_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Return(expression) => {
                self.assemble_expression(*expression);
                self.output.push("return".to_owned());
            },
        };
    }

    pub fn assemble_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Literal(literal) => {
                self.assemble_literal(*literal);
            },
        };
    }

    pub fn assemble_literal(&mut self, literal: Literal) {
        match literal {
            Literal::Int64(num) => {
                self.output.push(format!("i32.const {}", num));
            },
            Literal::Float64(num) => {
                self.output.push(format!("f32.const {}", num));
            },
        };
    }
}
