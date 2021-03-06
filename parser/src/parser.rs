use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::ast;
use super::lexer;
use super::python;

pub fn read_file(filename: &Path) -> Result<String, String> {
    match File::open(&filename) {
        Ok(mut file) => {
            let mut s = String::new();

            match file.read_to_string(&mut s) {
                Err(why) => Err(String::from("Reading file failed: ") + why.description()),
                Ok(_) => Ok(s),
            }
        }
        Err(why) => Err(String::from("Opening file failed: ") + why.description()),
    }
}

/*
 * Parse python code.
 * Grammar may be inspired by antlr grammar for python:
 * https://github.com/antlr/grammars-v4/tree/master/python3
 */

pub fn parse(filename: &Path) -> Result<ast::Program, String> {
    info!("Parsing: {}", filename.display());
    match read_file(filename) {
        Ok(txt) => {
            debug!("Read contents of file: {}", txt);
            parse_program(&txt)
        }
        Err(msg) => Err(msg),
    }
}

pub fn parse_program(source: &String) -> Result<ast::Program, String> {
    let lxr = lexer::Lexer::new(&source);
    match python::ProgramParser::new().parse(lxr) {
        Err(why) => Err(String::from(format!("{:?}", why))),
        Ok(p) => Ok(p),
    }
}

pub fn parse_statement(source: &String) -> Result<ast::Statement, String> {
    let lxr = lexer::Lexer::new(&source);
    match python::StatementParser::new().parse(lxr) {
        Err(why) => Err(String::from(format!("{:?}", why))),
        Ok(p) => Ok(p),
    }
}

pub fn parse_expression(source: &String) -> Result<ast::Expression, String> {
    let lxr = lexer::Lexer::new(&source);
    match python::ExpressionParser::new().parse(lxr) {
        Err(why) => Err(String::from(format!("{:?}", why))),
        Ok(p) => Ok(p),
    }
}

#[cfg(test)]
mod tests {
    use super::ast;
    use super::parse_program;
    use super::parse_statement;

    #[test]
    fn test_parse_print_hello() {
        let source = String::from("print('Hello world')\n");
        let parse_ast = parse_program(&source).unwrap();
        assert_eq!(
            parse_ast,
            ast::Program {
                statements: vec![
                    ast::Statement::Expression {
                        expression: ast::Expression::Call {
                            function: Box::new(ast::Expression::Identifier {
                                name: String::from("print"),
                            }),
                            args: vec![
                                ast::Expression::String {
                                    value: String::from("Hello world"),
                                },
                            ],
                        },
                    },
                ],
            }
        );
    }

    #[test]
    fn test_parse_print_2() {
        let source = String::from("print('Hello world', 2)\n");
        let parse_ast = parse_program(&source).unwrap();
        assert_eq!(
            parse_ast,
            ast::Program {
                statements: vec![
                    ast::Statement::Expression {
                        expression: ast::Expression::Call {
                            function: Box::new(ast::Expression::Identifier {
                                name: String::from("print"),
                            }),
                            args: vec![
                                ast::Expression::String {
                                    value: String::from("Hello world"),
                                },
                                ast::Expression::Number { value: 2 },
                            ],
                        },
                    },
                ],
            }
        );
    }

    #[test]
    fn test_parse_if_elif_else() {
        let source = String::from("if 1: 10\nelif 2: 20\nelse: 30\n");
        let parse_ast = parse_statement(&source).unwrap();
        assert_eq!(
            parse_ast,
            ast::Statement::If {
                test: ast::Expression::Number { value: 1 },
                body: vec![
                    ast::Statement::Expression {
                        expression: ast::Expression::Number { value: 10 },
                    },
                ],
                orelse: Some(vec![
                    ast::Statement::If {
                        test: ast::Expression::Number { value: 2 },
                        body: vec![
                            ast::Statement::Expression {
                                expression: ast::Expression::Number { value: 20 },
                            },
                        ],
                        orelse: Some(vec![
                            ast::Statement::Expression {
                                expression: ast::Expression::Number { value: 30 },
                            },
                        ]),
                    },
                ]),
            }
        );
    }
}
