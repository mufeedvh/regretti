use super::{
    lexer::{Lexer, Analyser},
    tokens::*,
    builtins::{Function, Builtin},
    states::ProgramState,
    memory::{MemoryLayout, Manager},
    messages::*,
};

pub struct Parser {
    pub(super) token: Token,
    pub(super) slice: String,
}

use std::process;
use std::str;

impl Parser {
    pub fn parse(&self) {
        match self.token {
            Token::Variable => {
                let token_set = Lexer::tokenize(&Lexer, &self.slice);
                let slice_set = Lexer::slice(&Lexer, &self.slice);

                let mut key: &str = &self.slice;
                let mut assigned: bool = false;
                let mut pointer = 0;

                if token_set.contains(&Token::Assign) {
                    assigned = true;
                }

                for token in token_set {
                    match token {
                        Token::Variable => (),
                        Token::Assign => (),
                        Token::Keyword => key = &slice_set[pointer],
                        token => {
                            if assigned {
                                let value = &slice_set[pointer];

                                MemoryLayout::alloc(
                                    key.to_string(),
                                    value.to_string(),
                                    token,
                                )
                            } else {
                                let state = ProgramState::read_state();
                                push_error(
                                    format!("Syntax error in `let` assign in line no: {}.", state.line)
                                );
                                process::exit(1)
                            }
                        },
                    }

                    pointer += 1;
                }
            },
            // SIDE NOTE FOR ME: keywords can be parsed like variables as well. (TODO maybe)
            Token::Keyword => {
                let token_set = Lexer::tokenize(&Lexer, &self.slice);
                let slice_set = Lexer::slice(&Lexer, &self.slice);

                // value set
                let keyword: String;
                let value: String;

                // creating value set
                // `token_set[1]` is the argument to a keyword
                match token_set[1] {
                    Token::Keyword => {
                        let keyword = &slice_set[0];
                        let value = self.slice.replace(keyword, "");

                        let mem_return = MemoryLayout::fetch(&value.trim());

                        if mem_return.is_some() {
                            let function = Function {
                                keyword: keyword.trim().to_string(),
                                value: mem_return.unwrap().value.to_string()
                            };
                            Function::execute(&function)
                        } else {
                            push_error(
                                format!("`{}` is not initialized/defined.", value)
                            );
                            process::exit(1)
                        }
                    }
                    Token::String => {
                        keyword = STRING.replace_all(&self.slice, "").to_string();
                        value = self.slice.replace(&keyword, "");
                        let function = Function {
                            keyword: keyword.trim().to_string(),
                            value,
                        };
                        Function::execute(&function)
                    },
                    // handle math expression here
                    _ => (),
                }
            },
            Token::Statement => {
                // a statement should only be executed inside a command block of course!
                let state = ProgramState::read_state();
                // verify command block state
                if state.function == Token::CommandStart {
                    // parse command
                    let statement = self.slice.replace("|", "");
                    // a statement contains a keyword and it's arguments
                    let parser = Self {
                        token: Token::Keyword,
                        slice: statement.trim().to_string(),
                    };
                    // execute keyword functions via recursive parsing
                    parser.parse()
                } else {
                    push_error("You cannot execute statements outside comments.".to_string());
                    process::exit(1)
                }
            }
            _ => (),
        }
    }
}