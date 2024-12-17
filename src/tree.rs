use std::collections::HashMap;

use crate::lexer::{Lexeme, Token, VarToken};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynaType {
    type_name: String,
}

#[derive(Debug, Clone)]
struct Block {
    block: Vec<Lexeme>,
}

#[derive(Debug, Clone)]
pub struct DynaFunction {
    pub name: String,
    body: Block,
    pub signature: HashMap<String, DynaType>,
}

#[derive(Debug, Clone)]
struct DynaEnum {
    name: String,
    fields: HashMap<String, Option<DynaType>>,
}

#[derive(Debug, Clone)]
struct DynaStruct {
    name: String,
    fields: HashMap<String, DynaType>,
}

#[derive(Debug, Clone)]
pub struct DynaCall {
    pub fn_name: String,
    pub signature: Vec<DynaType>,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Structure(DynaStruct),
    Enumeration(DynaEnum),
    Function(DynaFunction),
    FunctionCall(DynaCall),
    Block(Box<Ast>),
    Genesis,
}

pub fn tree(lexemes: Vec<Lexeme>) -> Vec<Ast> {
    let mut tree = vec![Ast::Block(Box::new(Ast::Genesis))];
    let mut peekable = lexemes.iter().peekable();

    while let Some(token) = peekable.peek() {
        match token {
            Lexeme::VariableByteToken(var_token) => match var_token {
                VarToken::Fn => {
                    peekable.next();
                    if let Some(Lexeme::TokenLiteral(fn_name)) = peekable.peek() {
                        peekable.next();
                        let mut signature = HashMap::new();
                        if let Some(Lexeme::ByteToken(Token::OpenParen)) = peekable.peek() {
                            peekable.next();
                            if let Some(Lexeme::TokenLiteral(argument)) = peekable.peek() {
                                peekable.next();

                                if let Some(Lexeme::ByteToken(Token::Colon)) = peekable.peek() {
                                    peekable.next();
                                } else {
                                    panic!("bad function syntax no colon after arg name");
                                }

                                if let Some(Lexeme::TokenLiteral(type_arg)) = peekable.peek() {
                                    peekable.next();
                                    let argument =
                                        std::str::from_utf8(argument).unwrap().to_string();
                                    let type_name =
                                        std::str::from_utf8(type_arg).unwrap().to_string();

                                    signature.insert(argument, DynaType { type_name });
                                } else {
                                    panic!("bad function syntax no arg type after arg name");
                                }
                            }
                            assert_eq!(
                                peekable.next(),
                                Some(&Lexeme::ByteToken(Token::CloseParen))
                            );
                        } else {
                            panic!("bad function syntax no open paren after fn name");
                        }

                        if let Some(Lexeme::ByteToken(Token::OpenCurly)) = peekable.peek() {
                            peekable.next();
                            let mut block = Vec::new();

                            while Some(&&Lexeme::ByteToken(Token::CloseCurly)) != peekable.peek() {
                                if let Some(x) = peekable.next() {
                                    block.push(x.clone())
                                }
                            }

                            let name = std::str::from_utf8(fn_name).unwrap().to_string();

                            let functor = DynaFunction {
                                name,
                                body: Block { block },
                                signature,
                            };

                            tree.push(Ast::Function(functor));
                        }
                    }
                }
                VarToken::Struct => {
                    peekable.next();
                    if let Some(Lexeme::TokenLiteral(struct_name)) = peekable.peek() {
                        peekable.next();
                        if let Some(Lexeme::ByteToken(Token::OpenCurly)) = peekable.peek() {
                            peekable.next();

                            let mut struct_fields = HashMap::new();

                            while let Some(Lexeme::TokenLiteral(struct_field)) = peekable.peek() {
                                let field_name = flush_whitespace(struct_field);
                                peekable.next();

                                if let Some(Lexeme::ByteToken(Token::Colon)) = peekable.peek() {
                                    peekable.next();
                                    if let Some(Lexeme::TokenLiteral(struct_field_type)) =
                                        peekable.peek()
                                    {
                                        peekable.next();
                                        let type_name = flush_whitespace(struct_field_type);

                                        struct_fields.insert(field_name, DynaType { type_name });

                                        if let Some(Lexeme::ByteToken(Token::Comma)) =
                                            peekable.peek()
                                        {
                                            peekable.next();

                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }

                            assert_eq!(
                                peekable.next(),
                                Some(&Lexeme::ByteToken(Token::CloseCurly))
                            );

                            let name = flush_whitespace(struct_name);

                            let structure = DynaStruct {
                                name,
                                fields: struct_fields,
                            };

                            tree.push(Ast::Structure(structure));
                        }
                    }
                }
                VarToken::Enum => {
                    peekable.next();
                    if let Some(Lexeme::TokenLiteral(enum_name)) = peekable.peek() {
                        peekable.next();
                        if let Some(Lexeme::ByteToken(Token::OpenCurly)) = peekable.peek() {
                            peekable.next();

                            let mut enum_fields = HashMap::new();

                            while let Some(Lexeme::TokenLiteral(enum_field)) = peekable.peek() {
                                let field_name = flush_whitespace(enum_field);
                                peekable.next();

                                if let Some(Lexeme::ByteToken(Token::OpenParen)) = peekable.peek() {
                                    peekable.next();
                                    if let Some(Lexeme::TokenLiteral(enum_field_type)) =
                                        peekable.peek()
                                    {
                                        peekable.next();
                                        let type_name = flush_whitespace(enum_field_type);

                                        enum_fields.insert(field_name, Some(DynaType { type_name }));

                                        assert_eq!(
                                            peekable.next(),
                                            Some(&Lexeme::ByteToken(Token::CloseParen))
                                        );

                                        if let Some(Lexeme::ByteToken(Token::Comma)) =
                                            peekable.peek()
                                        {
                                            peekable.next();

                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                } else {
                                    enum_fields.insert(field_name, None);
                                }
                            }

                            let syn_check = peekable.next();
                            match syn_check {
                                Some(&Lexeme::ByteToken(Token::CloseCurly))
                                | Some(&Lexeme::ByteToken(Token::Comma)) => (),
                                _ => panic!("expected comma or close curly"),
                            };

                            let name = flush_whitespace(enum_name);

                            let enumeration = DynaEnum {
                                name,
                                fields: enum_fields,
                            };

                            tree.push(Ast::Enumeration(enumeration));
                        }
                    }
                }
                _ => {
                    peekable.next();
                }
            },
            Lexeme::TokenLiteral(token) => {
                peekable.next();
                if let Some(Lexeme::ByteToken(Token::OpenParen)) = peekable.peek() {
                    peekable.next();

                    let mut types = Vec::new();
                    if let Some(Lexeme::TokenLiteral(call_arg_type)) = peekable.peek() {
                        peekable.next();
                        types.push(DynaType {
                            type_name: flush_whitespace(call_arg_type)
                        });
                    }

                    assert_eq!(peekable.next(), Some(&Lexeme::ByteToken(Token::CloseParen)));

                    tree.push(Ast::FunctionCall(DynaCall {
                        fn_name: flush_whitespace(token),
                        signature: types
                    }))
                }
            }
            _ => {
                peekable.next();
            }
        }
    }

    tree
}

fn flush_whitespace(literal: &[u8]) -> String {
    let string = std::str::from_utf8(literal).unwrap().to_string();

    string.replace(" ", "")
}
