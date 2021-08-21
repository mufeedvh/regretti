use super::{
    lexer::{Lexer, Analyser},
    tokens::*,
    builtins::{Function, Builtin},
    states::ProgramState
};

pub struct Parser {
    pub(super) token: Token,
    pub(super) slice: String,
}

impl Parser {
    pub fn parse(&self) {
        match self.token {
            Token::Keyword => {
                let token_set = Lexer::tokenize(&Lexer, &self.slice);

                // value set
                let keyword: String;
                let value: String;

                // creating value set
                match token_set[1] {
                    Token::String => {
                        keyword = STRING.replace_all(&self.slice, "").to_string();
                        value = self.slice.replace(&keyword, "").replace("\"", "");
                        let function = Function {
                            keyword: keyword.trim().to_string(),
                            value,
                        };
                        Function::execute(&function)
                    },
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
                    // execute keyword functions
                    parser.parse()
                }
            }
            _ => (),
        }
    }
}