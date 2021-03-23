use std::collections::HashMap;
use std::result::Result;

pub type Handler = fn(args: HashMap<String, Argument>) -> Result<i32, String>;

pub type Lib = HashMap<String, Func>;

#[derive(Clone)]
pub struct Func {
    pub args: HashMap<String, Argument>,
    pub handler: Handler,
}

#[derive(Clone)]
pub struct Argument {
    pub required: bool,
    pub value: String,
}


pub fn get_arg(name: &str, args: HashMap<String, Argument>) -> String {
    args.get(name).expect("Very very bad error this should not happen").value.clone()
}


pub fn empty_func() -> Func {
    Func {
        args: HashMap::new(),
        handler: empty_handler,
    }
}

fn empty_handler(_args: HashMap<String, Argument>) -> Result<i32, String> {
    Ok(0)
}
