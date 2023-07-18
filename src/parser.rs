use std::iter::Peekable;

use crate::ast::{Program, Declaration, Function, Scope, Argument, TypeAnnotation, Statement, Expression, Literal};

struct Parser<I> where I: Iterator<Item = String> {
    tokens: Peekable<I>
}

impl <I> Parser<I> where I: Iterator<Item = String> {
    fn has_more_tokens(&mut self) -> bool {
        self.tokens.peek().is_some()
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
        let mut result = vec![];

        while self.has_more_tokens() {
            let type_annotation_identifier = self.tokens.next().expect(
                "Functions should have a type annotation",
            );

            let type_annotation = Box::new(
                TypeAnnotation {
                    identifier: type_annotation_identifier,
                },
            );

            let identifier = self.tokens.next().expect(
                "Functions should have a name.",
            );

            let decider = self.tokens.peek().expect(
                "Declaration signature should be followed by a token.",
            );

            match decider.as_str() {
                "(" => {
                    let function = self.parse_function(identifier, type_annotation);
                    let declaration = Declaration::Function(Box::new(function));
                    result.push(declaration);
                },
                _ => {
                    panic!("Declaration signature is followed by invalid token.");
                }
            }
        }

        result
    }

    fn parse_function(&mut self, identifier: String, type_annotation: Box<TypeAnnotation>) -> Function {
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

        Function {
            identifier,
            type_annotation,
            arguments,
            body,
        }
    }

    fn parse_argument(&mut self) -> Argument {
        unimplemented!()
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
            "return" => {
                let _ = self.tokens.next().unwrap();
                let expression = self.parse_expression();
                let terminator = self.tokens.next().expect(
                    "Statement should be terminated.",
                );

                if terminator.as_str() != ";" {
                    panic!("Statement should be terminated.");
                }

                Statement::Return(Box::new(expression))
            },
            _ => {
                println!("dada: {}", self.tokens.next().unwrap());
                panic!(
                    "Invalid statement."
                );
            }
        }
    }

    fn parse_expression(&mut self) -> Expression {
        Expression::Literal(
            Box::new(
                self.parse_literal()
            )
        )
    }

    fn parse_literal(&mut self) -> Literal {
        let literal = self.tokens.next().expect(
            "Literal should not be empty."
        );

        if literal.contains(".") {
            Literal::Float64(literal.parse().expect(
                "Literal not parseable as a float.",
            ))
        } else {
            Literal::Int64(literal.parse().expect(
                "Literal not parseable as an int.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::ast::{Program, Declaration, Function, TypeAnnotation, Scope, Statement, Expression, Literal};
    use crate::parser::Parser;

    #[test]
    fn tokenize_simple_program_correctly() {
        let tokens_text = fs::read_to_string("samples/01-simple/tokens.txt").unwrap();
        let tokens: Vec<_> = tokens_text.split("\n").collect();
        let main_function = Function {
            identifier: "main".to_owned(),
            type_annotation: Box::new(
                TypeAnnotation {
                    identifier: "int".to_owned(),
                },
            ),
            arguments: vec![],
            body: Box::new(
                Scope {
                    statements: vec![
                        Statement::Return(
                            Box::new(
                                Expression::Literal(
                                    Box::new(
                                        Literal::Int64(
                                            1
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ],
                },
            ),
        };
        let program = 
            Box::new(
                Program {
                    declarations: vec![
                        Declaration::Function(
                            Box::new(
                                main_function,
                            ),
                        ),
                    ],
                },
            );
        assert_eq!(Parser::parse(tokens.into_iter().map(|s| s.to_owned())), program);
    }
}
