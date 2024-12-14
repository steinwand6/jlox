use std::{collections::HashMap, rc::Rc};

use crate::{
    token::{Object, Token},
    LoxRuntimeError,
};

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Rc<Environment>) -> Self {
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
                Some(enclosing) => enclosing.get(name),
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
        } else {
            match &self.enclosing {
                Some(enclosing) => match enclosing.get(name) {
                    Ok(_) => self.define(&name.lexeme, value),
                    Err(e) => return Err(e),
                },
                None => {
                    return Err(LoxRuntimeError(
                        name.clone(),
                        format!("Undefined variable '{}'.", name.lexeme),
                    ))
                }
            }
        }
        Ok(())
    }
}
