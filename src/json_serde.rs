use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use Value::{Arr, Bool, Null, Number, Obj, Str};

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    Number(f64),
    Arr(Vec<Value<'a>>),
    Obj(HashMap<Value<'a>, Value<'a>>),
}

impl<'a> Eq for Value<'a> {}

impl<'a> Hash for Value<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Str(v) => v.hash(state),
            Null => unimplemented!(),       // hash only needed for string
            Bool(_v) => unimplemented!(),   // hash only needed for string
            Number(_v) => unimplemented!(), // hash only needed for string
            Arr(_v) => unimplemented!(),    // hash only needed for string
            Obj(_v) => unimplemented!(),    // hash only needed for string
        }
    }
}
