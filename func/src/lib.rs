use std::collections::HashMap;
use std::result::Result;

pub struct Func {
    pub name: String,
    pub args: HashMap<String, Argument>,
    pub handler: Box<dyn Handler>,
}

pub struct Argument {
    pub required: bool,
    pub value: String,
}

pub trait Handler {
    fn handle(&self, args: HashMap<String, Argument>) -> Result<i32, String>;

    fn check(&self, _args: HashMap<String, Argument>) -> Result<i32, String> {
        return Ok(0);
    }
}

pub type Lib = Vec<Func>;
