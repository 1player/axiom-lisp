use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use crate::expr::Expr;

struct EnvInner {
    parent: Option<Rc<RefCell<EnvInner>>>,
    slots: HashMap<String, Expr>,
}

impl EnvInner {
    pub fn new(parent: Option<Rc<RefCell<EnvInner>>>) -> Self {
        Self {
            parent,
            slots: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.slots
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().map(|p| p.borrow().get(name)).flatten())
    }

    pub fn set(&mut self, name: &str, value: Expr) {
        self.slots.insert(name.to_owned(), value);
    }
}

pub struct Env {
    e: Rc<RefCell<EnvInner>>,
}

impl Env {
    pub fn new(parent: Option<&Env>) -> Self {
        let parent = parent.map(|p| p.e.clone());

        Self {
            e: Rc::new(RefCell::new(EnvInner::new(parent))),
        }
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.e.borrow().get(name)
    }

    pub fn set(&mut self, name: &str, value: Expr) {
        self.e.borrow_mut().set(name, value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_set() {
        let mut env = Env::new(None);
        assert!(env.get("foo").is_none());

        env.set("foo", Expr::new_symbol("bar"));
        assert_eq!(env.get("foo").unwrap(), Expr::new_symbol("bar"));
    }

    #[test]
    fn test_nested() {
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
