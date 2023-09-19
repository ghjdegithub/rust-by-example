// #![allow(unused)]
//
// fn main() {
//     // `F` 必须是泛型的。
//     fn apply<F>(f: F) where
//         F: FnOnce() {
//         f();
//     }
// }

// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的
// 要求恰好一样。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}
