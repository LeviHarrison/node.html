use std::collections::HashMap;
use std::result::Result;

use html5ever::{local_name, namespace_url, ns, QualName};

pub type Handle = usize;

#[derive(Clone)]
pub enum Node {
    IsElement(Element),
    IsText(Text),
}

#[derive(Clone)]
pub struct Text {
    pub value: String,
}

#[derive(Clone)]
pub struct Element {
    pub element_name: String,
    pub attributes: HashMap<String, String>,
    pub is_func: bool,
    pub func: Func,
    pub matched_attributes: HashMap<String, Argument>,
    pub children: Vec<Handle>,
    pub qual_name: QualName,
}

impl Element {
    pub fn new() -> Element {
        Element {
            element_name: String::new(),
            attributes: HashMap::new(),
            is_func: false,
            func: Func::new(),
            matched_attributes: HashMap::new(),
            children: Vec::new(),
            qual_name: QualName::new(None, ns!(html), local_name!("")),
        }
    }
}

pub type Handler = fn(args: HashMap<String, Argument>, body: Vec<Node>) -> Result<i32, String>;

pub type Elements = usize;

pub type Lib = HashMap<String, Func>;

#[derive(Clone)]
pub struct Func {
    pub args: HashMap<String, Argument>,
    pub require_body_types: bool,
    pub accepted_body: Accepted,
    pub handler: Handler,
}

#[derive(Clone)]
pub struct Argument {
    pub required: bool,
    pub value: String,
}

#[derive(Clone)]
pub struct Accepted {
    pub accept_text: bool,
    pub accepted_funcs: Vec<String>,
    pub accepted_elements: Vec<String>,
}

impl Func {
    pub fn new() -> Func {
        Func {
            args: HashMap::new(),
            require_body_types: false,
            accepted_body: Accepted::new(),
            handler: empty_handler,
        }
    }
}

impl Accepted {
    pub fn new() -> Accepted {
        Accepted {
            accept_text: true,
            accepted_funcs: Vec::new(),
            accepted_elements: Vec::new(),
        }
    }
}

pub fn get_arg(name: &str, args: HashMap<String, Argument>) -> String {
    args.get(name)
        .expect("Expected arg could not be found")
        .value
        .clone()
}

fn empty_handler(_args: HashMap<String, Argument>, _body: Vec<Node>) -> Result<i32, String> {
    Ok(0)
}
