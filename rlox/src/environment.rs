use std::collections::HashMap;

use crate::{
    token::{Object, Token},
    LoxRuntimeError,
};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxRuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxRuntimeError(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            )),
        }
    }
}
