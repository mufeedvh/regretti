use super::{
    lexer::{Lexer, Analyser},
    tokens::*,
    builtins::{Function, Builtin},
    states::ProgramState,
    memory::{MemoryLayout, Value, Manager},
    messages::*,
};

pub struct Parser {
    pub(super) token: Token,
    pub(super) slice: String,
}

use std::process;
use std::str;

impl Parser {
    // parsing is performed in a deterministic syntax tree lookup
    pub fn parse(&self) {
        match self.token {
            Token::Variable => {
                let token_set = Lexer::tokenize(&Lexer, &self.slice);
                let slice_set = Lexer::slice(&Lexer, &self.slice);

                let mut key: &str = &self.slice;
                let mut assigned: bool = false;
                let mut key_set: bool = false;
                let mut pointer = 0;

                if token_set.contains(&Token::Assign) {
                    assigned = true;
                }

                if token_set.contains(&Token::Math) {
                    let mut expression = String::new();
                    for token in token_set {
                        match token {
                            Token::Variable => (),
                            Token::Assign => (),
                            Token::Keyword => {
                                let value = &slice_set[pointer];
                                if !key_set {
                                    key = value;
                                    key_set = true;
                                } else {
                                    println!("{}", value);
                                    // handle variables and assemble math expression with their respective values
                                    if Lexer::tokenize(&Lexer, value)[0] == Token::Number {
                                        expression.push_str(value)
                                    } else {
                                        let mem_return = MemoryLayout::fetch(value);
                                        if mem_return.is_some() {
                                            let value = match mem_return.unwrap() {
                                                Value::String(value) => value,
                                                Value::FInt(value) => value.to_string(),
                                                Value::Int(value) => value.to_string(),
                                                Value::Nothing => unimplemented!(),
                                            };
                                            expression.push_str(&value)
                                        } else {
                                            push_error(
                                                format!("`{}` is not initialized.", value)
                                            );
                                            process::exit(1)
                                        }
                                    }
                                }
                            },
                            _ => {
                                if assigned {
                                    // get value
                                    let value = &slice_set[pointer];

                                    let token_type = Lexer::tokenize(&Lexer, value)[0];

                                    // handle variables and assemble math expression with their respective values
                                    if token_type == Token::Keyword {
                                        let mem_return = MemoryLayout::fetch(value);
                                        if mem_return.is_some() {
                                            let value = match mem_return.unwrap() {
                                                Value::String(value) => value,
                                                Value::FInt(value) => value.to_string(),
                                                Value::Int(value) => value.to_string(),
                                                Value::Nothing => unimplemented!(),
                                            };
                                            expression.push_str(&value)
                                        } else {
                                            push_error(
                                                format!("`{}` is not initialized.", value)
                                            );
                                            process::exit(1)
                                        }
                                    } else {
                                        expression.push_str(value)
                                    }
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

                    let ret = Function::math_evaluator(&expression).to_string();
                    
                    // allocate memory
                    MemoryLayout::alloc(
                        key.to_string(),
                        Value::String(ret),
                    )
                } else {
                    for token in token_set {
                        match token {
                            Token::Variable => (),
                            Token::Assign => (),
                            Token::Keyword => key = &slice_set[pointer],
                            _ => {
                                if assigned {
                                    // get value
                                    let value = &slice_set[pointer];
    
                                    // allocate memory
                                    MemoryLayout::alloc(
                                        key.to_string(),
                                        Value::String(value.to_string()),
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
                if token_set.len() < 1 {
                    return
                }
                match token_set[1] {
                    Token::Keyword => {
                        let keyword = &slice_set[0];
                        let value = self.slice.replace(keyword, "");
                        let value = &value.trim();

                        // evaluating math expressions executed from a keyword
                        if token_set.contains(&Token::Math) {
                            let mut construct = String::new();
                            let temp_eval = value.chars();
                            for c in temp_eval {
                                match c {
                                    '+' | '-' | '*' | '/' => {
                                        let append = format!(" {} ", c);
                                        construct.push_str(&append)
                                    },
                                    _ => construct.push(c)
                                }
                            }
                            let construct = construct.split(" ");
                            let mut expression = String::new();
                            for c in construct {
                                // denoting a math expression
                                match c {
                                    "+" | "-" | "*" | "/" | "" => expression.push_str(c),
                                    _ => {
                                        if Lexer::tokenize(&Lexer, c)[0] == Token::Number {
                                            expression.push_str(c)
                                        } else {
                                            let mem_return = MemoryLayout::fetch(c);
                                            if mem_return.is_some() {
                                                let value = match mem_return.unwrap() {
                                                    Value::String(value) => value,
                                                    Value::FInt(value) => value.to_string(),
                                                    Value::Int(value) => value.to_string(),
                                                    Value::Nothing => unimplemented!(),
                                                };
                                                expression.push_str(&value)
                                            } else {
                                                push_error(
                                                    format!("`{}` is not initialized.", value)
                                                );
                                                process::exit(1)
                                            }
                                        }
                                    }
                                }
                            }

                            // evaluate math expressions
                            let ret = Function::math_evaluator(&expression).to_string();

                            let function = Function {
                                keyword: keyword.trim().to_string(),
                                value: ret,
                            };

                            Function::execute(&function)
                        } else {
                            let mem_return = MemoryLayout::fetch(value);

                            if mem_return.is_some() {
                                let value = match mem_return.unwrap() {
                                    Value::String(value) => value,
                                    Value::FInt(value) => value.to_string(),
                                    Value::Int(value) => value.to_string(),
                                    Value::Nothing => unimplemented!(),
                                };
    
                                let function = Function {
                                    keyword: keyword.trim().to_string(),
                                    value
                                };
    
                                Function::execute(&function)
                            } else {
                                push_error(
                                    format!("`{}` is not initialized.", value)
                                );
                                process::exit(1)
                            }
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
                    Token::Number => {
                        let keyword = &slice_set[0];
                        let value = self.slice.replace(keyword, "");

                        // evaluate math expressions
                        let ret = Function::math_evaluator(&value).to_string();

                        let function = Function {
                            keyword: keyword.trim().to_string(),
                            value: ret,
                        };
                        Function::execute(&function)
                    }
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