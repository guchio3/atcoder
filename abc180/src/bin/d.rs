use proconio::{input, fastout};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
    }
    let base_num = min(b / (a - 1), y);
    let mut exp = x;
    let mut res = 0;
    loop {
        // overflow care
        if y / a < x { break; }

        if exp * a < base_num {
            res += 1;
            exp = exp * a;
        } else {
            break;
        }
    }
    // let mut res = usize::log(a, );
    // let after_a_exp = x + ;
    if exp < y {
        res += (y - exp - 1) / b;
    }

    println!("{}", res);
}


// use proconio::{input, fastout};
// // use std::cmp::min;
// 
// #[fastout]
// fn main() {
//     input! {
//         x: f64,
//         y: f64,
//         a: f64,
//         b: f64,
//         // x: usize,
//         // y: usize,
//         // a: usize,
//         // b: usize,
//     }
//     // let base_num = min(b / (a - 1), y);
//     // let mut exp = x;
//     // let mut res = 0;
//     // loop {
//     //     if exp * a < base_num {
//     //         res += 1;
//     //         exp *= a;
//     //     } else {
//     //         break;
//     //     }
//     // }
//     // let base_num = min(b / (a - 1), y - 1) as f64;
//     let mut base_num = b / (a - 1.);
//     if base_num > y - 1. {
//         base_num = y - 1.;
//     }
//     let mut res = (base_num / x).log(a) as usize;
//     // x*a^? = base_num; -> ?*log_a(a) = log_a(base_num / x)
//     let exp = (x * a).powf(res as f64);
// 
//     if exp < y {
//         res += ((y - exp - 1.) / b) as usize;
//     }
// 
//     println!("{}", res);
// }


// #[fastout]
// fn main() {
//     input! {
//         x: usize,
//         y: usize,
//         a: usize,
//         b: usize,
//     }
//     let mut strength = vec![0; y + 1];
//     let mut a_i = x+1;
//     let mut b_i = x+1;
//     let mut res = 0;
//     loop {
//         // loop して a_i, b_i の足並みを揃えつつ
//         // どちらも y をこえるまで進める
//         // どちらも y を超えた時点での max
//         if a_i > y {
//             if b_i > y {
//                 break;
//             } else {
//                 strength[b_i] = max(strength[b_i], );
//                 b_i ;
//                 b_i 
//             }
//         }
//     }
//     println!("{}", res);
// }
