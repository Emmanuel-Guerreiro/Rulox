use std::fmt::Display;
use std::ops;

use crate::interpreter::RuntimeError;
/*All this object abstraction is a workaround for the difficulties of
doing runtime checking of types in Rust
Is quite difficult in rust to return Any from a function and to cast on runtime
Box<dyn any> to the necessary type

So the solution is to */

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Object {
    StringObj(String),
    BoolObj(bool),
    NumberObj(f64),
    NullObj,
}

//This will be usefull when implementing control flow
// impl Object {
//     pub fn is_truthy(&self) -> bool {
//         match self {
//             Object::BoolObj(v) => *v,
//             Object::NumberObj(n) => *n != 0.0,
//             Object::StringObj(s) => !s.is_empty(),
//             _ => unimplemented!(
//                 "Can not use objects of type {:?} as boolean expression.",
//                 self
//             ),
//         }
//     }
// }

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::BoolObj(v) => f.write_fmt(format_args!("{}", v.to_string())),
            Object::NumberObj(v) => f.write_fmt(format_args!("{}", v)),
            Object::StringObj(v) => f.write_fmt(format_args!("{}", v)),
            Object::NullObj => f.write_str(""),
        }
    }
}

impl ops::Add<Object> for Object {
    type Output = Result<Object, RuntimeError>;
    fn add(self, rhs: Self) -> Result<Object, RuntimeError> {
        let res = match (&self, &rhs) {
            (Object::NumberObj(a), Object::NumberObj(b)) => Object::NumberObj(a + b),
            (Object::StringObj(a), Object::StringObj(b)) => {
                //Todo: How efficient is this?
                let mut res = String::with_capacity(a.len() + b.len());
                res.push_str(a);
                res.push_str(b);
                Object::StringObj(res)
            }
            //I could implement String + number operation to coerse number into
            //string and append the strings. But im not such a mounster
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Unsuported operand for sum {:?} {:?}",
                    self, rhs
                )))
            }
        };

        Ok(res)
    }
}

impl ops::Sub for Object {
    type Output = Result<Object, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Object::NumberObj(a), Object::NumberObj(b)) => Object::NumberObj(a - b),
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Unsuported operand for substraction {:?} {:?}",
                    self, rhs
                )))
            }
        };

        Ok(res)
    }
}

impl ops::Mul for Object {
    type Output = Result<Object, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Object::NumberObj(a), Object::NumberObj(b)) => Object::NumberObj(a * b),
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Unsuported operand for substraction {:?} {:?}",
                    self, rhs
                )))
            }
        };

        Ok(res)
    }
}

impl ops::Div for Object {
    type Output = Result<Object, RuntimeError>;
    fn div(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Object::NumberObj(a), Object::NumberObj(b)) => Object::NumberObj(a / b),
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Unsuported operand for substraction: {:?} {:?}",
                    self, rhs
                )))
            }
        };

        Ok(res)
    }
}

// - Operator
impl ops::Neg for Object {
    type Output = Result<Object, RuntimeError>;
    fn neg(self) -> Self::Output {
        let res = match self {
            Object::NumberObj(v) => Object::NumberObj(-v),
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Unsuported operand for negation: {:?}",
                    self
                )))
            }
        };

        Ok(res)
    }
}

//Bang
impl ops::Not for Object {
    type Output = Result<Object, RuntimeError>;
    fn not(self) -> Self::Output {
        let res = match self {
            Object::BoolObj(v) => Object::BoolObj(!v),
            //There is no bitwise operation because the numbers are all floats
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Unsuported operand for Not operation: {:?}",
                    self
                )))
            }
        };

        Ok(res)
    }
}

//This macro and the following impl for Object implements
//The comparison operations for the object
macro_rules! impl_cmp {
    ($func_name:ident, $op:tt) => {
        pub fn $func_name(&self, other: &Self) -> Result<Object, RuntimeError> {

            if std::mem::discriminant(self) != std::mem::discriminant(other) {
                return Err(RuntimeError::TypeError(format!(
                    "Comparison is not supported between {:?} {:?}",
                    self, other
                )));
            }

            Ok(Object::BoolObj(self $op other))
        }
    };
}

impl Object {
    impl_cmp!(gt, >);
    impl_cmp!(gte, >=);
    impl_cmp!(lt, <);
    impl_cmp!(lte, <=);

    pub fn eq(&self, other: &Self) -> Result<Object, RuntimeError> {
        Ok(Object::BoolObj(self == other))
    }

    pub fn neq(&self, other: &Self) -> Result<Object, RuntimeError> {
        Ok(Object::BoolObj(self != other))
    }
}
