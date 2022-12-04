use std::collections::HashMap;

use crate::expr::Expr;

pub struct Env {
    slots: HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.slots.get(name)
    }

    pub fn set(&mut self, name: &str, value: Expr) {
        self.slots.insert(name.to_owned(), value);
    }
}
