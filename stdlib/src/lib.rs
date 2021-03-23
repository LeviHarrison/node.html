use func::{Argument, Func, Lib, get_arg};

use std::collections::HashMap;

fn hello(_args: HashMap<String, Argument>) -> Result<i32, String> {
    println!("Hello");
    Ok(0)
}

fn print(args: HashMap<String, Argument>) -> Result<i32, String> {
    println!("{}", get_arg("string", args));
    Ok(0)
}

pub fn register() -> Lib {
    let mut lib = HashMap::new();

    lib.insert(String::from("hello"), Func {
        args: HashMap::new(),
        handler: hello,
    });

    let mut args = HashMap::new();
    args.insert(String::from("string"), Argument {required: true, value: String::new()});

    lib.insert(String::from("print"), Func {
        args: args,
        handler: print,
    });

    lib
}
