//
// #![allow(unused)]
// fn main() {
//     let my_str = "hello";
//     let my_string = String::from(my_str);
// }

// use std::convert::From;
//
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }
//
// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }
//
// fn main() {
//     let num = Number::from(30);
//     println!("My number is {:?}", num);
// }

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
