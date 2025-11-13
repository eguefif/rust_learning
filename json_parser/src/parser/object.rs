use std::ops::Index;
use crate::JsonType;

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
