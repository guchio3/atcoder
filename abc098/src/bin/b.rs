use proconio::{input, fastout};
use proconio::marker::Chars;
use std::collections::HashSet;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, s: String
    }

    let mut res = 0;

    for i in 1..(n-1) {
        let x: HashSet<char> = s[..i].chars().collect();
        let y: HashSet<char> = s[i..].chars().collect();

        let intersect = x.intersection(&y).count();
        res = max(intersect, res);
    }

    println!("{}", res);
}

// #[fastout]
// fn main() {
//     input! {
//         n: usize, s: Chars
//     }
// 
//     let mut res = 0;
// 
//     for i in 1..(n-1) {
//         let x: HashSet<char> = s[..i].to_vec().iter().map(|c| *c).collect();
//         let y: HashSet<char> = s[i..].to_vec().iter().map(|c| *c).collect();
// 
//         let intersect: Vec<char> = x.intersection(&y).into_iter().map(|c| *c).collect();
//         res = max(intersect.len(), res);
//     }
// 
//     println!("{}", res);
// }
