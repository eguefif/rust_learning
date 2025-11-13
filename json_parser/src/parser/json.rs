use std::ops::Index;
use super::num::Num;
pub use super::object::Object;

#[derive(Debug, PartialEq)]
pub enum JsonType {
    Str(String),
    Num(Num),
    Bool(bool),
    Object(Box<Object>),
    Array(Vec<JsonType>)
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
