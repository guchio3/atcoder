use std::io::stdin;
use std::str::FromStr;
use std::cmp::max;
use std::cmp::min;

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

fn read_str_as_char_vec() -> Vec<char> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().chars().collect()
}

fn read_str_as_T<T>() -> T 
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().unwrap()
}

// fn binary_search(x: usize, a: usize, b: usize, n: usize) -> usize {
//     if x == 0 || x == n {
//         let left = a * x / b;
//         let right = a * (x / b);
//         left - right;
//     }
//     let left_x = x - 1;
//     let right_x = x + 1;
// 
//     let res = a * x / b - a * (x / b);
//     let left_res = a * left_x / b - a * (left_x / b);
//     let right_res = a * right_x / b - a * (right_x / b);
//     if left_res > res {
//         ;
//     }
// 
//     max(binary_search(), ())
// }

fn main() {
    let in_vec: Vec<usize> = read_line();
    let a = in_vec[0];
    let b = in_vec[1];
    let n = in_vec[2];

    let mut res = 0;
    let mut x: usize = 0;
    let fin = min(b, n);
    while x <= fin {
        // println!("{}", x);
        res = max(res, a * x / b - a * (x / b));
        x += max(min((b - a * x % b) / a, b - x % b), 1);
//         if x == 0 {
//             x += 1;
//             continue;
//         } else {
//             if max(b, a * x) >= min(a * x, b) {
//                 if b >= x{
//                     x += min(b - a * x % b, b - x % b);
//                     continue;
//                 }
//             }
//             break;
//             // x += min(max(b - a * x, 1), max(b - x, 1));
//         }
    }
    // for x in 0..=min(b, n) {
    //     let left = a * x / b;
    //     let right = a * (x / b);
    //     res = max(res, left - right);
    // }
    
    println!("{}", res);

    //let temp = 1_000_000_000_000 - (1_000_000_000_000 % b);
//    let temp = n - (n % b) - 1;
    // let x:usize;
    // let left:usize;
    // let right:usize;
    // if n >= n % b + 1 {
    //     x = n - (n % b) - 1;
    //     left = (a * x) / b;
    //     right = a * (x / b);
    //     println!("AA");
    // } else {
    //     let x_1 = 1;
    //     let left_1 = (a * x_1) / b;
    //     let right_1 = a * (x_1 / b);

    //     let x_2 = n;
    //     let left_2 = (a * x_1) / b;
    //     let right_2 = a * (x_1 / b);

    //     if left_1 - right_1 < left_2 - right_2 {
    //         left = left_2;
    //         right = right_2;
    //     println!("BB");
    //     } else {
    //         left = left_1;
    //         right = right_1;
    //     println!("CC");
    //     }
    // }
    // println!("{}, {}", left, right);
    // println!("{}", left - right);
}
