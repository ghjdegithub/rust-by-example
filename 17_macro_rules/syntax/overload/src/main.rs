// 根据你调用它的方式，`test!` 将以不同的方式来比较 `$left` 和 `$right`。
#[deny(clippy::eq_op)]
macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right)
    };
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => {
        println!("{:?} and {:?} is {:?}",
               stringify!($left),
               stringify!($right),
               $left || $right)
    }
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; and false);
}
