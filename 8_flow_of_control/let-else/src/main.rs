// #![allow(unused)]
// fn main() {
//     use std::str::FromStr;
//
//     fn get_count_item(s: &str) -> (u64, &str) {
//         let mut it = s.split(' ');
//         let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
//             panic!("invalid input: {}", s);
//         };
//         let Ok(count) = u64::from_str(count_str) else {
//             panic!("invalid count: {}", count_str);
//         };
//         (count, item)
//     }
//
//     assert_eq!(get_count_item("3 chairs"), (3, "chairs"))
// }

#![allow(unused)]

fn main() {
    use std::str::FromStr;

    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (count_str, item) = match (it.next(), it.next()) {
            (Some(count_str), Some(item)) => (count_str, item),
            _ => panic!("invalid input: {}", s),
        };
        let count = if let Ok(count) = u64::from_str(count_str) {
            count
        } else {
            panic!("invalid count: {}", count_str);
        };
        (count, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"))
}
