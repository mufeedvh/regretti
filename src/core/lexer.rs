use std::process;

use logos::Logos;

use super::{
    tokens::*,
    messages::*,
    states::ProgramState,
    parser::Parser,
};

pub struct Lexer;

pub trait Analyser {
    fn analyse(&self, slice: &str) -> Option<Token>;
    fn tokenize(&self, slice: &str) -> Vec<Token>;
    fn lexerize(&self, source: &str);
}

/// Lexical Analysis
impl Analyser for Lexer {
    // stepped analysis
    fn analyse(&self, slice: &str) -> Option<Token> {
        // statement grammar
        match token_grammer(slice) {
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

    // tokenizer
    // `tokenize()` essentially does the same thing `lexerize()` does
    // -> individual instructions are analysed on a smaller subset of tokens for fewer amount of lookups.
    fn tokenize(&self, slice: &str) -> Vec<Token> {
        let mut lex = Token::lexer(slice);
        let mut tokens: Vec<Token> = Vec::with_capacity(2);

        loop {
            let (token, _span, slice) = (lex.next(), lex.span(), lex.slice());

            if token.is_some() {
                match token {
                    Some(Token::PassLex) => {
                        let token = self.analyse(slice);
                        tokens.push(token.unwrap())
                    },
                    _ => tokens.push(token.unwrap())
                }
            } else {
                break
            }
        }

        tokens
    }

    // main lexer
    fn lexerize(&self, source: &str) {
        let mut lex = Token::lexer(source);
        let mut line: usize = 1;
    
        loop {
            let (token, _span, slice) = (lex.next(), lex.span(), lex.slice());
    
            // println!("{:?}", slice);
            // println!("{:?}", token);
    
            match token {
                // line coint
                Some(Token::Newline) => line += 1,
                // entry point
                Some(Token::MainFunction) => {
                    ProgramState::set_state(
                        Token::MainFunction,
                        None,
                        line
                    )
                },
                // end of a function
                Some(Token::FunctionEnd) => {
                    ProgramState::set_state(
                        Token::FunctionEnd,
                        None,
                        line
                    )
                },            
                // start of a comand
                Some(Token::CommandStart) => {
                    ProgramState::set_state(
                        Token::CommandStart,
                        None,
                        line
                    )                
                },
                // end of a command
                Some(Token::CommandEnd) => {
                    ProgramState::set_state(
                        Token::CommandEnd,
                        None,
                        line
                    )                
                },
                // manual parsing
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