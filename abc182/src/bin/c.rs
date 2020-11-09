use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut res = 2_147_483_647i32;
    for i in 0..i32::pow(2, n.len() as u32) {
        let mut temp_sum = 0;
        let mut temp_res = n.len() as i32;
        for j in 0..n.len() {
            if (1 << j) & i == 0 {
                temp_sum += n[j].to_digit(10).unwrap();
                temp_res -= 1;
            }
        }
        if temp_res == (n.len() as i32) {
            continue;
        }
        if temp_sum % 3 == 0 {
            if temp_res < res {
                res = temp_res;
            }
        }
    }

    if res == 2_147_483_647i32 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}


// #[fastout]
// fn main() {
//     input! {
//         n: Chars,
//     }
//     let mut digit_sum = 0;
//     for &n_i in n.iter() {
//         digit_sum += n_i.to_digit(10).unwrap();
//     }
// 
//     let mut res = 0;
//     for n_i in n.iter().rev() {
//         if digit_sum % 3 == 0 {
//             println!("{}", res);
//             return;
//         } else {
//             digit_sum -= n_i.to_digit(10).unwrap();
//             res += 1;
//         }
//     }
//     
//     println!("-1");
// }
