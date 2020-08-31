use Value::{Bool, Null, Number, Str};

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    Number(f64),
    Arr(Vec<Value<'a>>),
}

// impl PartialEq for Value {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Null, Null) => true,
//             (Bool(v1), Bool(v2)) => v1 == v2,
//             (Str(v1), Str(v2)) => v1 == v2,
//             (Number(v1), Number(v2)) => v1 == v2,
//         }
//     }
// }
