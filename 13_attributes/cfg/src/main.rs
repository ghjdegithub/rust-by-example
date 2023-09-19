// 这个函数仅当目标系统是 macos 的时候才会编译
#[cfg(target_os = "macos")]
fn are_you_on_macos() {
    println!("You are running macos!");
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "macos"))]
fn are_you_on_macos() {
    println!("You are *not* running macos!");
}

fn main() {
    are_you_on_macos();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely macos!");
    } else {
        println!("Yes. It's definitely *not* macos!");
    }
}
