// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// 你可以使用` use `关键字将任何地方的模块路径，特别是Rust标准库中的模块路径引入到你的作用域中。
// 从std::time模块中获取SystemTime和UNIX_EPOCH。如果你能用一行就能做到，那就有额外的风格加分!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.


// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
