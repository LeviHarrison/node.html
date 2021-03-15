use func::{Argument, Func, Lib};

use std::collections::HashMap;

fn hello(_args: HashMap<String, Argument>) -> Result<i32, String> {
    println!("Hello");
    Ok(0)
}

pub fn register() -> Lib {
    let mut lib = Vec::new();

    lib.push(Func {
        name: String::from("Hello"),
        args: HashMap::new(),
        handler: hello,
    });

    lib
}
