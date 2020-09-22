use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 1..=n {
        res += ((i + (n/i)*i) * (n/i)) / 2
    }
    // for i in 1..=n {
    //     for j in 1..=n {
    //         if i % j == 0 {
    //             res += i;
    //         }
    //     }
    // }
    println!("{}", res);
}

// fn calc_multiple_num(base: usize, upper_limit: usize) -> usize {
//     ;
// }

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//     }
//     let mut res = 0;
//     for k in 1..=n {
//         res += k * f(k);
//     }
//     println!("{}", res);
// }
// 
// fn f(k: usize) -> usize {
//     let mut res = 0;
//     let mut upper_bound = usize::MAX;
//     for i in 1..=k {
//         if i >= upper_bound {
//             break;
//         }
//         if k % i == 0 {
//             res += 1;
//             if k / i == i {
//                 break;
//             } else {
//                 res += 1;
//                 upper_bound = k / i;
//             }
//         }
//     }
//     res
// }
