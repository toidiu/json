#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    String(String),
    Integer(u32),
}
