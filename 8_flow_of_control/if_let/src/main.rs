// #![allow(unused)]
// fn main() {
//     // 将 `optional` 定为 `Option<i32>` 类型
//     let optional = Some(7);
//
//     match optional  {
//         Some(i)=>{
//             println!("This is a really long string and `{:?}`",i);
//             // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
//             // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
//         },
//         _=>{},
//         // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
//     }
// }

// fn main() {
//     // 全部都是 `Option<i32>` 类型
//     let number = Some(7);
//     let letter: Option<i32> = None;
//     let emoticon: Option<i32> = None;
//
//     // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
//     // 语句块（`{}`）
//     if let Some(i) = number {
//         println!("Matched {:?}!", i);
//     }
//
//     // 如果要指明失败情形，就使用 else：
//     if let Some(i) = letter {
//         println!("Matched {:?}!", i);
//     } else {
//         // 解构失败。切换到失败情形。
//         println!("Didn't match a number. Let's go with a letter!");
//     }
//
//     // 提供另一种失败情况下的条件。
//     let i_like_letters = false;
//
//     if let Some(i) = emoticon {
//         println!("Matched {:?}!", i);
//         // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
//     } else if i_like_letters {
//         println!("Didn't match a number. Let's go with a letter!");
//     } else {
//         // 条件的值为 false。于是以下是默认的分支：
//         println!("I don't like letters. Let's go with an emoticon :)!");
//     }
// }

// // 以这个 enum 类型为例
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32),
// }
//
// fn main() {
//     // 创建变量
//     let a = Foo::Bar;
//     let b = Foo::Baz;
//     let c = Foo::Qux(100);
//
//     // 变量 a 匹配到了 Foo::Bar
//     if let Foo::Bar = a {
//         println!("a is foobar");
//     }
//
//     // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
//     if let Foo::Bar = b {
//         println!("b is foobar");
//     }
//
//     // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
//     if let Foo::Qux(value) = c {
//         println!("c is {}", value);
//     }
// }

// 该枚举故意未注明 `#[derive(PartialEq)]`，
// 并且也没为其实现 `PartialEq`。这就是为什么下面比较 `Foo::Bar==a` 会失败的原因。
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
        // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        println!("a is foobar")
    }
}
