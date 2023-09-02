use crate::object::Object;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum EnviromentError {
    UndefinedVariable(String),
    UnknownError,
}

impl Display for EnviromentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnviromentError::UndefinedVariable(s) => {
                write!(f, "Variable {} is undefined", s)
            }
            Self::UnknownError => {
                write!(f, "Unkown error with scopes")
            }
        }
    }
}

type VarContent = Box<Object>;
type EnviromentResult<'a> = Result<VarContent, EnviromentError>;

pub struct Enviroment {
    values: HashMap<String, VarContent>,
}

impl Enviroment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> EnviromentResult {
        if let Some(v) = self.values.get(name) {
            // This is ugly
            return Ok(v.clone());
        }
        Err(EnviromentError::UndefinedVariable(name.to_string()))
    }

    pub fn define(&mut self, name: String, value: Option<Box<Object>>) {
        let insert_value = match value {
            None => Box::new(Object::NullObj),
            Some(o) => o,
        };

        self.values.insert(name, insert_value);
    }

    //Util function to debug
    pub fn print_status(&self) {
        println!("{:?}", self.values);
    }
}
