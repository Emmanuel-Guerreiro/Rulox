use crate::{interpreter::RuntimeError, object::Object};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

//This is an ugly solution to the difficulties of building LLs in rust
type VarContent = Object;
type EnviromentResult = Result<VarContent, RuntimeError>;
type EnviromentOption = Option<VarContent>;
#[derive(Debug, Default)]
pub struct EnvironmentInner {
    locals: HashMap<String, VarContent>,
}

#[derive(Debug, Clone)]
pub struct Environment {
    envs: Vec<Rc<RefCell<EnvironmentInner>>>,
    curr: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            envs: Vec::from([Rc::new(RefCell::new(EnvironmentInner::default()))]),
            curr: 0,
        }
    }

    //Will create a new local env to the stack, and move the pointer to it.
    //Any following variable operation will start with it
    pub fn add_new_local(&mut self) -> Result<(), RuntimeError> {
        let new_env = Rc::new(RefCell::new(EnvironmentInner::default()));
        self.envs.push(new_env);
        //The last one (The one at the top) is the innermost scope
        self.curr = self.envs.len() - 1;
        // print!("New local -> ");
        // self.print_status();
        Ok(())
    }

    pub fn remove_local(&mut self) -> Result<(), RuntimeError> {
        if self.envs.len() == 1 {
            return Err(RuntimeError::ScopeError(Some(String::from(
                "Cant remove the global scope",
            ))));
        }

        self.envs.pop();
        //Reset the current scope to the innermost
        self.curr += self.envs.len();
        Ok(())
    }

    pub fn define(&mut self, name: &String, value: Option<Object>) -> Result<(), RuntimeError> {
        let insertion_value = match value {
            Some(v) => v,
            None => Object::NullObj,
        };

        self.envs[self.curr]
            .borrow_mut()
            .locals
            .insert(name.clone(), insertion_value);
        // print!("Define variable -> ");
        // self.print_status();
        Ok(())
    }

    pub fn assign(&mut self, name: &String, value: Object) -> EnviromentResult {
        let inner = self.envs[self.curr].borrow();
        while self.curr < self.envs.len() {
            match inner.locals.get(name) {
                Some(_) => match self.envs[self.curr]
                    .borrow_mut()
                    .locals
                    .insert(name.clone(), value)
                {
                    None => return Err(RuntimeError::UndefinedVariable(name.clone())),
                    Some(a) => {
                        // self.print_status();
                        return Ok(a);
                    }
                },
                None => {
                    if self.curr == 0 || self.envs.len() == 1 {
                        return Err(RuntimeError::UndefinedVariable(name.clone()));
                    }
                    self.curr -= 1;
                }
            }
        }
        return Err(RuntimeError::UndefinedVariable(name.clone()));
    }

    pub fn get(&self, name: &String) -> EnviromentOption {
        let loc_curr = self.envs.len();
        for l in (0..loc_curr).rev() {
            // println!("Envs: {:?}", self.envs[l]);
            if let Some(v) = self.envs[l].borrow().locals.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn print_status(&self) {
        println!("Enviroment status: {:?}", self.envs);
    }
}
