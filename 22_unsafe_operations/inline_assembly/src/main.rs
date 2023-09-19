// #![allow(unused)]
//
// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         unsafe {
//             asm!("nop");
//         }
//     }
// }

// # ![allow(unused)]
//
// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         let x: u64;
//         unsafe {
//             asm!("mov {}, 5", out(reg) x);
//         }
//         assert_eq!(x, 5);
//     }
// }

// #![allow(unused)]
//
// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         let i: u64 = 3;
//         let o: u64;
//         unsafe {
//             asm!(
//                 "mov {0}, {1}",
//                 "add {0}, {1}, 5",
//                 out(reg) o,
//                 in(reg) i,
//             );
//         }
//         assert_eq!(o, 8);
//     }
// }

// #![allow(unused)]
//
// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         let mut x: u64 = 3;
//         unsafe {
//             asm!("add {0}, {0}, 5", inout(reg) x);
//         }
//         assert_eq!(x, 8);
//     }
// }

// #![allow(unused)]
//
// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         let mut a: u64 = 4;
//         let b: u64 = 4;
//         let c: u64 = 4;
//         unsafe {
//             asm!(
//             "add {0}, {0}, {1}",
//             "add {0}, {0}, {2}",
//             inout(reg) a,
//             in(reg) b,
//             in(reg) c,
//             );
//         }
//         assert_eq!(a, 12);
//     }
// }

// #![allow(unused)]
//
// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         let mut a: u64 = 4;
//         let b: u64 = 4;
//         unsafe {
//             asm!(
//             "add {0}, {0}, {1}",
//             inlateout(reg) a,
//             in(reg) b,
//             );
//         }
//         assert_eq!(a, 8);
//     }
// }

// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         let cmd = 0xd1;
//         unsafe {
//             asm!(
//             "out 0x64, eax",
//             in("eax") cmd,
//             );
//         }
//     }
// }

// fn main() {
//     #[cfg(target_arch = "aarch64")]{
//         use std::arch::asm;
//
//         fn mul(a: u64, b: u64) -> u128 {
//             let lo: u64;
//             let hi: u64;
//
//             unsafe {
//                 asm!(
//                 // The x86 mul instruction takes rax as an implicit input and writes
//                 // the 128-bit result of the multiplication to rax:rdx.
//                 "mul {}",
//                 in(reg) a,
//                 inlateout("rax") b => lo,
//                 lateout("rdx") hi
//                 );
//             }
//
//             ((hi as u128) << 64) + lo as u128
//         }
//     }
// }

#![allow(unused)]
fn main() {
    #[cfg(target_arch = "aarch64")] {
        use std::arch::asm;

        let mut a: u64 = 4;
        let b: u64 = 4;
        unsafe {
            asm!(
            "add {0}, {1}",
            inlateout(reg) a, in(reg) b,
            options(pure, nomem, nostack),
            );
        }
        assert_eq!(a, 8);
    }
}
