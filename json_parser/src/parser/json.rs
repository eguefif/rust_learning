use std::ops::Index;
use super::num::Num;

#[derive(Debug, PartialEq)]
pub enum JsonType {
    Str(String),
    Num(Num),
    Bool(bool),
    Object(Box<Object>),
}

#[derive(Debug)]
pub struct Json {
    pub(crate) data: Object,
}

impl Index<&str> for Json {
    type Output = JsonType;

    fn index<'a, 'b>(&'a self, index: &'b str) -> &'a Self::Output {
        &self.data[index]
    }
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub(crate) data: Vec<(String, JsonType)>,
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
