use super::tokens::*;

use parking_lot::Mutex;
use std::collections::HashMap;
use ahash::RandomState;
use once_cell::sync::Lazy;
use copystr::s32;

#[derive(Copy, Clone)]
pub struct MemoryLayout {
    pub(super) value: s32,
    pub(super) datatype: Token,
}

pub trait Manager {
    fn fetch(key: &str) -> Option<MemoryLayout>;
    fn alloc(key: String, value: String, datatype: Token);
}

// better assume: String(&'static str)

pub static MEMORY_MAP: Lazy<Mutex<HashMap<String, MemoryLayout, RandomState>>> =
    Lazy::new(|| {
        Mutex::new(HashMap::default())
    });

impl Manager for MemoryLayout {
    fn fetch(key: &str) -> Option<Self> {
        let memory_map = &MEMORY_MAP;

        if memory_map.lock().contains_key(key) {
            Some(*memory_map.lock().get(key).unwrap())
        } else {
            None
        }
    }

    fn alloc(key: String, value: String, datatype: Token) {
        let state = Self {
            value: s32::new(&value).unwrap(),
            datatype,
        };
        
        MEMORY_MAP.lock().insert(key, state);
    }
}