use std::process;

use logos::Logos;

use super::{
    tokens::*,
    messages::*,
    states::{ProgramState, Operation},
    parser::Parser,
};

pub struct Lexer;

pub trait Analyser {
    fn analyse(&self, slice: &str) -> Option<Token>;
    fn tokenize(&self, slice: &str) -> Vec<Token>;
    fn slice(&self, slice: &str) -> Vec<String>;
    fn lexerize(&self, source: &str);
}

/// Lexical Analysis
impl Analyser for Lexer {
    // stepped analysis
    fn analyse(&self, slice: &str) -> Option<Token> {
        // statement grammar
        match token_grammar(slice) {
            Token::Statement => {
                let slice_parse = Parser {
                    token: Token::Statement,
                    slice: slice.to_string(),
                };
                Parser::parse(&slice_parse);
                None
            },
            Token::String => Some(Token::String),
            _ => None
        }
    }

    fn tokenize(&self, slice: &str) -> Vec<Token> {
        let mut lex = Token::lexer(slice);
        // maximum statement scope stays 4
        let mut tokens: Vec<Token> = Vec::with_capacity(4);

        loop {
            let (token, _span, slice) = (lex.next(), lex.span(), lex.slice());

            if token.is_some() {
                match token {
                    Some(Token::PassLex) => {
                        let token = self.analyse(slice);
                        if token.is_some() {
                            tokens.push(token.unwrap())   
                        }
                    },
                    _ => tokens.push(token.unwrap())
                }
            } else {
                break
            }
        }

        tokens
    }

    // umm... why?? (debug)
    fn slice(&self, slice: &str) -> Vec<String> {
        let mut lex = Token::lexer(slice);
        // maximum statement scope stays 4
        let mut slices: Vec<String> = Vec::with_capacity(4);

        loop {
            let (token, slice) = (lex.next(), lex.slice());

            if token.is_some() {
                match token {
                    Some(Token::PassLex) => {
                        let token = self.analyse(slice);
                        if token.is_some() {
                            slices.push(slice.to_string())
                        }
                    },
                    _ => slices.push(slice.to_string())
                }
            } else {
                break
            }
        }

        slices
    }    

    // main lexer
    fn lexerize(&self, source: &str) {
        let mut lex = Token::lexer(source);
        let mut line: usize = 1;
    
        loop {
            let (token, _span, slice) = (lex.next(), lex.span(), lex.slice());
    
            // println!("{:?} :: {:?}", slice, token);
    
            match token {
                // line coint
                Some(Token::Newline) => line += 1,
                // entry point
                Some(Token::MainFunction) => {
                    ProgramState::set_state(
                        Token::MainFunction,
                        Operation::StateChange,
                        line
                    )
                },
                // end of a function
                Some(Token::FunctionEnd) => {
                    ProgramState::set_state(
                        Token::FunctionEnd,
                        Operation::StateChange,
                        line
                    )
                },            
                // start of a comand
                Some(Token::CommandStart) => {
                    ProgramState::set_state(
                        Token::CommandStart,
                        Operation::StateChange,
                        line
                    )                
                },
                // end of a command
                Some(Token::CommandEnd) => {
                    ProgramState::set_state(
                        Token::CommandEnd,
                        Operation::StateChange,
                        line
                    )
                },
                Some(Token::Variable) => {
                    // set state to allocation
                    ProgramState::set_state(
                        Token::Variable,
                        Operation::Allocation,
                        line
                    );

                    // take full slice
                    let mut scope_lex = lex.clone();
                    let mut scope_slice = String::new();

                    // add variable key
                    scope_slice.push_str(slice);
                    scope_slice.push(' ');

                    loop {
                        // get arbitrary syntax
                        let (token, slice) = (scope_lex.next(), scope_lex.slice());

                        // read until end of statement
                        if token != Some(Token::Newline) {
                            scope_slice.push_str(slice);
                            scope_slice.push(' ')
                        } else {
                            break
                        }
                    }

                    // pass the complete slice to parser
                    let slice_parse = Parser {
                        token: Token::Variable,
                        slice: scope_slice,
                    };

                    Parser::parse(&slice_parse);
                },
                // stepped parsing
                Some(Token::PassLex) => { self.analyse(slice); },
                // finish point
                None => process::exit(0),
                // nope!
                _ => (),
            }
    
            // main syntax validation
            if line == 1 && token != Some(Token::MainFunction) {
                push_error("Program should start with a main function.".to_string());
                process::exit(1)
            }        
        }
    }
}