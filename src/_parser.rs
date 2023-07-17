use std::iter::Peekable;

use crate::ast::{Program, Declaration, Function, Scope, Argument, Variable, TypeAnnotation, Statement, VariableDeclaration, Expression};

struct Parser<I> where I: Iterator<Item = String> {
    tokens: Peekable<I>
}

impl <I> Parser<I> where I: Iterator<Item = String> {
    fn has_more_tokens(&mut self) -> bool {
        return self.tokens.peek().is_some()
    }

    fn parse(tokens: I) -> Box<Program> {
        let mut parser = Parser {
            tokens: tokens.peekable(),
        };
        
        Box::new(parser.parse_program())
    }

    fn parse_program(&mut self) -> Program {
        let declarations = self.parse_declarations();
        Program {
            declarations,
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let result = vec![];

        while self.has_more_tokens() {
            let token = self.tokens.next().unwrap();
            match token.as_str() {
                "function" => {
                    let declaration = self.parse_function();
                },
                _ => {
                    panic!("Wrong declaration keyword.");
                }
            }
        }

        result
    }

    fn parse_function(&mut self) -> Function {
        let identifier = self.tokens.next().expect(
            "Functions should have a name."
        );

        let mut arguments = vec![];

        let open_parens = self.tokens.next().expect(
            "Functions should have an argument list."
        );
        if open_parens.as_str() != "(" {
            panic!("Arguments of a function call should be wrapped in parenthesis.");
        }

        while self.has_more_tokens() && self.tokens.peek().unwrap().as_str() != ")" {
            arguments.push(self.parse_argument());
        }

        let close_parens = self.tokens.next().expect(
            "Functions should have an argument list."
        );
        if close_parens.as_str() != ")" {
            panic!("Arguments of a function call should be wrapped in parenthesis.");
        }

        let body = Box::new(self.parse_scope());

        Function { identifier, arguments, body }
    }

    fn parse_argument(&mut self) -> Argument {
        let variable_identifier = self.tokens.next().expect(
            "Argument variable name should be present.",
        );

        let variable = Box::new(
            Variable { identifier: variable_identifier }
        );

        let separator = self.tokens.next().expect(
            "Function argument should have a separator."
        );
        if separator.as_str() != ":" {
            panic!("Function argument should have a separator.");
        }

        let type_annotation_identifier = self.tokens.next().expect(
            "Argument type annotation name should be present.",
        );

        let type_annotation = Box::new(
            TypeAnnotation { identifier: type_annotation_identifier }
        );

        Argument { variable, type_annotation }
    }

    fn parse_scope(&mut self) -> Scope {
        let scope_open = self.tokens.next().expect(
            "Scopes should be opened with a curly braces."
        );

        if scope_open.as_str() != "{" {
            panic!("Scopes should be opened with a curly braces.");
        }

        let mut statements = vec![];

        while self.has_more_tokens() && self.tokens.peek().unwrap().as_str() != "}" {
            statements.push(self.parse_statement());
        }


        let scope_close = self.tokens.next().expect(
            "Scopes should be closed with a curly braces."
        );

        if scope_close.as_str() != "}" {
            panic!("Scopes should be closed with a curly braces.");
        }

        Scope { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.tokens.peek().unwrap().as_str() {
            "var" => {
                let variable_declaration = self.parse_variable_declaration();
                Statement::VariableDeclaration(Box::new(variable_declaration))
            },
            "while" => {
                unimplemented!()
            },
            _ => {
                let expression = self.parse_expression();
                Statement::Expression(Box::new(expression))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        let keyword = self.tokens.next().expect(
            "Variable declarations should start with the keyword var."
        );

        if keyword.as_str() != "var" {
            panic!(
                "Variable declarations should start with the keyword var."
            )
        }

        let variable = Box::new(self.parse_variable());

        let equal_sign = self.tokens.next().expect(
            "Variable declarations should be followed by an assignment operator.",
        );

        if equal_sign.as_str() != "=" {
            panic!(
                "Variable declarations should be followed by an assignment operator."
            );
        }

        let expression = Box::new(self.parse_expression());

        VariableDeclaration { variable, expression }
    }

    // FIXME: IMPLEMENT PARSER
    fn parse_expression(&mut self) -> Expression {
        let next_token = self.tokens.peek().expect(
            "Expressions should not be empty."
        );

        match next_token.as_str() {
            | "+"
            | "-"
            | "*"
            | "/"
            | "^"
            | ">"
            | ">="
            | "<"
            | "<="
            | "<<"
            | ">>"
            | "&&"
            | "||"
            | "%%"
            | "=" => {
                panic!("An operator cannot be the first token in an expression.")
            }
            "(" => {
                unimplemented!()
            },
            _ => {
                if next_token.as_str() == "\"" {

                }

                unimplemented!()
            }
        }
    }

    fn parse_variable(&mut self) -> Variable {
        let identifier = self.tokens.next().expect(
            "Variables should have an identifier."
        );

        Variable { identifier  }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, ast::{Declaration, Program, Function, Argument, Variable, TypeAnnotation, Scope, Statement, Expression, OperatorCall, OperatorIdentifier, Literal, VariableDeclaration}};

    #[test]
    fn tokenize_basic_program_correctly() {
        let tokens = vec![
            "function", "main", "(", "args", ":", "Array<String>", ")", "{",
            "var", "x", "=", "14.0", ";",
            "var", "y", "=", "x", "^", "2", ";",
            "var", "z", "=", "x", "+", "x", "*", "y", ";",
            "}", ";"
        ];
        let x_expression =
            Expression::Literal(
                Box::new(
                    Literal::Float64(14.0),
                )
            );
        let y_expression = Expression::OperatorCall(
            Box::new(
                OperatorCall {
                    operator_identifier: OperatorIdentifier::Exponential,
                    lhs: Box::new(
                        Expression::Variable(
                            Box::new(
                                Variable {
                                    identifier: "x".to_owned(),
                                },
                            ),
                        ),
                    ),
                    rhs: Box::new(
                        Expression::Literal(
                            Box::new(
                                Literal::Int64(2),
                            )
                        )
                    ),
                }
            )
        );
        let z_expression = Expression::OperatorCall(
            Box::new(
                OperatorCall {
                    operator_identifier: OperatorIdentifier::Add,
                    lhs: Box::new(
                        Expression::Variable(
                            Box::new(
                                Variable {
                                    identifier: "x".to_owned(),
                                },
                            )
                        )
                    ),
                    rhs: Box::new(
                        Expression::OperatorCall(
                            Box::new(
                                OperatorCall {
                                    operator_identifier: OperatorIdentifier::Multiply,
                                    lhs: Box::new(
                                        Expression::Variable(
                                            Box::new(
                                                Variable {
                                                    identifier: "x".to_owned(),
                                                },
                                            )
                                        ),
                                    ),
                                    rhs: Box::new(
                                        Expression::Variable(
                                            Box::new(
                                                Variable {
                                                    identifier: "y".to_owned(),
                                                },
                                            )
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                }
            )
        );
        let main_function_body = Scope {
            statements: vec![
                Statement::VariableDeclaration(
                    Box::new(
                        VariableDeclaration {
                            variable: Box::new(
                                Variable { identifier: "x".to_owned() }
                            ),
                            expression: Box::new(
                                x_expression,
                            ),
                        },
                    ),
                ),
                Statement::VariableDeclaration(
                    Box::new(
                        VariableDeclaration {
                            variable: Box::new(
                                Variable { identifier: "y".to_owned() }
                            ),
                            expression: Box::new(
                                y_expression,
                            ),
                        },
                    ),
                ),
                Statement::VariableDeclaration(
                    Box::new(
                        VariableDeclaration {
                            variable: Box::new(
                                Variable { identifier: "z".to_owned() }
                            ),
                            expression: Box::new(
                                z_expression,
                            ),
                        },
                    ),
                ),
            ],
        };
        let main_function = Declaration::Function(
            Box::new(
                Function {
                    identifier: "main".to_owned(),
                    arguments: vec![
                        Argument {
                            variable: Box::new(
                                Variable {
                                    identifier: "args".to_owned(),
                                },
                            ),
                            type_annotation: Box::new(
                                TypeAnnotation {
                                    identifier: "Array<String>".to_owned()
                                }
                            ),
                        },
                    ],
                    body: Box::new(
                        main_function_body,
                    ),
                },
            )
        );
        let program = 
            Box::new(
                Program {
                    declarations: vec![
                        main_function,
                    ],
                }
            );
        assert_eq!(Parser::parse(tokens.into_iter().map(|s| s.to_owned())), program);
    }
}
