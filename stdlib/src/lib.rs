use core::{get_arg, Accepted, Argument, Func, Lib, Node};
use Node::{IsElement, IsText};

use std::collections::HashMap;

fn hello(_args: HashMap<String, Argument>, _body: Vec<Node>) -> Result<i32, String> {
    println!("Hello");
    Ok(0)
}

fn print(args: HashMap<String, Argument>, _body: Vec<Node>) -> Result<i32, String> {
    println!("{}", get_arg("string", args));
    Ok(0)
}

fn test_body(_args: HashMap<String, Argument>, body: Vec<Node>) -> Result<i32, String> {
    println!("I was passed these nodes:");
    for node in body {
        match node {
            IsElement(e) => println!("{}", e.element_name),
            IsText(t) => println!("{}", t.value),
        }
    }
    println!("That's all!");
    Ok(0)
}

pub fn register() -> Lib {
    let mut lib = HashMap::new();

    lib.insert(
        String::from("hello"),
        Func {
            handler: hello,
            ..Func::new()
        },
    );

    let mut args = HashMap::new();
    args.insert(
        String::from("string"),
        Argument {
            required: true,
            value: String::new(),
        },
    );

    lib.insert(
        String::from("print"),
        Func {
            args,
            handler: print,
            ..Func::new()
        },
    );

    lib.insert(
        String::from("testbody"),
        Func {
            handler: test_body,
            require_body_types: true,
            accepted_body: Accepted {
                accept_text: true,
                ..Accepted::new()
            },
            ..Func::new()
        },
    );

    lib
}
