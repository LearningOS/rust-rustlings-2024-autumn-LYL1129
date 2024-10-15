// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// DONE

// TODO: Complete this use statement
/*引入std::time模块中的SystemTime和UNIX_EPOCH常量*/
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    /*获取当前的系统时间,计算从 Unix 纪元（1970年1月1日 00:00:00 UTC）到当前时间的持续时间。*/
    match SystemTime::now().duration_since(UNIX_EPOCH) { 
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
