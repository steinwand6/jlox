use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    token::{Object, Token},
    LoxRuntimeError,
};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.into(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxRuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(LoxRuntimeError(
                    name.clone(),
                    format!("Undefined variable '{}'.", name.lexeme),
                )),
            },
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), LoxRuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }
        Err(LoxRuntimeError(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn drop_enclosing(&mut self) {
        self.enclosing = None;
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            enclosing: self.enclosing.clone(),
        }
    }
}
