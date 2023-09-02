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

    //Add new variable to enviroment
    pub fn define(&mut self, name: String, value: Option<Box<Object>>) {
        let insert_value = match value {
            None => Box::new(Object::NullObj),
            Some(o) => o,
        };

        self.values.insert(name, insert_value);
    }

    //Update, if exists, variable in the enviroment
    pub fn assign(&mut self, variable: &String, value: Box<Object>) -> EnviromentResult {
        match self.values.get(variable) {
            None => Err(EnviromentError::UndefinedVariable(format!(
                "Variable {} is not defined",
                variable
            ))),
            Some(_) => Ok(self.values.insert(variable.clone(), value).unwrap()),
        }
    }

    //Util function to debug
    pub fn print_status(&self) {
        println!("{:?}", self.values);
    }
}
