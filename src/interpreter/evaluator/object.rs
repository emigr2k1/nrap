use core::fmt::Pointer;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Float(f64),
    String(String),
    Bool(bool),
    Array(HashMap<Vec<u64>, Box<Object>>),
    None,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::String(string) => write!(f, "{}", string),
            Object::Float(float) => write!(f, "{}", float),
            Object::Bool(boolean) => write!(f, "{}", boolean),
            Object::Array(array) => array.fmt(f),
            Object::None => writeln!(f, "None"),
        }
    }
}