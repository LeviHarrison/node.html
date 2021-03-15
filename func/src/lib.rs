use std::collections::HashMap;
use std::result::Result;

pub type Handler = fn(args: HashMap<String, Argument>) -> Result<i32, String>;

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub args: HashMap<String, Argument>,
    pub handler: Handler,
}

#[derive(Clone)]
pub struct Argument {
    pub required: bool,
    pub value: String,
}

pub type Lib = Vec<Func>;

pub fn empty() -> Func {
    Func {
        name: String::new(),
        args: HashMap::new(),
        handler: empty_func,
    }
}

fn empty_func(_args: HashMap<String, Argument>) -> Result<i32, String> {
    Ok(0)
}
