// pub mod my_io {
//     use std::io::stdin;
//     use std::str::FromStr;
//
//     pub fn read_line<T>() -> Vec<T>
//     where
//         T: FromStr,
//         <T as std::str::FromStr>::Err: std::fmt::Debug,
//     {
//         let mut s = String::new();
//         stdin().read_line(&mut s).ok();
//         s.trim()
//             .split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect()
//     }
// }
//
// use my_io::read_line;

use std::io::stdin;
use std::str::FromStr;

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let res: Vec<i32> = read_line();
    println!("{} {} {}", res[2], res[0], res[1]);
}
