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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut env = Env::new();
        assert!(env.get("foo").is_none());

        env.set("foo", Expr::Symbol("bar".to_owned()));
        assert_eq!(env.get("foo"), Some(&Expr::Symbol("bar".to_owned())));
    }
}
