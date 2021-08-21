use parking_lot::Mutex;
use std::collections::HashMap;

use ahash::RandomState;
use once_cell::sync::Lazy;

use super::tokens::Token;

#[derive(Copy, Clone)]
pub struct ProgramState {
    pub(crate) function: Token,
    pub(crate) operation: Option<Token>,
    pub(crate) line: usize,
}

pub const STATE_KEY: &str = "state";

pub static PROGRAM_STATE: Lazy<Mutex<HashMap<&str, ProgramState, RandomState>>> =
    Lazy::new(|| {
        let hasher = RandomState::new();
        Mutex::new(HashMap::with_capacity_and_hasher(1, hasher))
    });

impl ProgramState {
    pub fn read_state() -> Self {
        *PROGRAM_STATE.lock().get(STATE_KEY).unwrap()
    }

    pub fn set_state(function: Token, operation: Option<Token>, line: usize) {
        let state = Self {
            function,
            operation,
            line,
        };
        PROGRAM_STATE.lock().insert(STATE_KEY, state);
    }
}