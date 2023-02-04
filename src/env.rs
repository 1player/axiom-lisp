use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::expr::Expr;

struct EnvInner {
    parent: Option<Rc<RefCell<EnvInner>>>,
    slots: HashMap<String, Expr>,
}

impl EnvInner {
    fn new(parent: Option<Rc<RefCell<EnvInner>>>) -> Self {
        Self {
            parent,
            slots: HashMap::new(),
        }
    }

    fn get(&self, name: &str) -> Option<Expr> {
        self.slots
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().map(|p| p.borrow().get(name)).flatten())
    }

    fn set(&mut self, name: &str, value: Expr) {
        self.slots.insert(name.to_owned(), value);
    }
}

// The public wrapper to hide the whole Rc<RefCell> dance
// and have a more ergonomic API
pub struct Env(Rc<RefCell<EnvInner>>);

impl Env {
    pub fn new(parent: Option<&Env>) -> Self {
        let parent = parent.map(|p| p.0.clone());
        Self(Rc::new(RefCell::new(EnvInner::new(parent))))
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.0.borrow().get(name)
    }

    pub fn set(&mut self, name: &str, value: Expr) {
        self.0.borrow_mut().set(name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_environments() {
        let mut parent = Env::new(None);
        let mut child = Env::new(Some(&parent));

        parent.set("shared", Expr::new_symbol("parent"));
        parent.set("parent-unique", Expr::new_symbol("123"));

        child.set("shared", Expr::new_symbol("child"));
        child.set("child-unique", Expr::new_symbol("456"));

        assert_eq!(parent.get("shared"), Some(Expr::new_symbol("parent")));
        assert_eq!(parent.get("parent-unique"), Some(Expr::new_symbol("123")));
        assert_eq!(parent.get("child-unique"), None);

        assert_eq!(child.get("shared"), Some(Expr::new_symbol("child")));
        assert_eq!(child.get("parent-unique"), Some(Expr::new_symbol("123")));
        assert_eq!(child.get("child-unique"), Some(Expr::new_symbol("456")));
    }
}
