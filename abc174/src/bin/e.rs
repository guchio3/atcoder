// use ordered_float::NotNan;
use proconio::input;
// use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, mut k: usize,
        a: [f64; n]
    }
    let mut res = 0.;
    let mut l = 0.;
    let mut r = 1_000_000_000.;
    loop {
        let x = (l + r) / 2.;
        let mut k_sum = 0;
        for &a_i in a.iter() {
            k_sum += (a_i / x).ceil() as usize - 1;
        }
        if k_sum <= k {
            res = x;
            if r == x {
                break;
            }
            r = x;
        } else {
            if l == x {
                break;
            }
            l = x;
        }
    }
    println!("{}", res.ceil());
}

// fn main() {
//     input! {
//         n: usize, mut k: usize,
//         a: [f64; n]
//     }
//     let mut bh = BinaryHeap::new();
//     for a_i in a {
//         bh.push((
//             NotNan::new(a_i).unwrap(), // current value
//             NotNan::new(a_i).unwrap(), // base value
//             NotNan::new(1.).unwrap(),  // how many times devided
//         ));
//     }
//
//     for _i in 0..k {
//         let mut peek_one = bh.pop().unwrap();
//         peek_one.2 += 1.;
//         peek_one.0 = peek_one.1 / peek_one.2;
//         bh.push(peek_one);
//     }
//
//     println!("{}", bh.peek().unwrap().0.ceil());
// }
