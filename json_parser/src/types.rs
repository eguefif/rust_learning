use crate::JsonType;
use std::ops::Index;

#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Integer(i64),
    Float(f64),
}

impl Num {
    pub fn serialize(&self) -> String {
        match self {
            Num::Integer(value) => value.to_string(),
            Num::Float(value) => value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub(crate) data: Vec<(String, JsonType)>,
}

impl Object {
    pub fn new(data: Vec<(String, JsonType)>) -> Self {
        Self { data }
    }
}

impl Index<&str> for Object {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: &'b str) -> &'a Self::Output {
        for (key, value) in &self.data {
            if key == index {
                return &value;
            }
        }
        panic!("Error: trying to dereference Json object with unknown key");
    }
}
