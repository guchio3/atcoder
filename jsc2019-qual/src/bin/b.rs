use proconio::{input, fastout};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,  k: usize,
        a: [usize; n]
    }

    // let mut period_cnts = vec![0; n];
    let mut period_sum = 0;
    let mut local_sum = 0;
    for i in 0..n {
        for j in 0..n {
            if a[i] > a[j] {
                // period_cnts[i] += 1;
                period_sum += 1;
                if i < j {
                    local_sum += 1;
                }
            }
        }
    }

    let res = (k * local_sum % MOD + (k * (k - 1) / 2) % MOD * period_sum % MOD) % MOD;
  
    println!("{}", res);
}

// #[fastout]
// fn main() {
//     input! {
//         n: usize,  k: usize,
//         a: [usize; n]
//     }
// 
//     // let mut period_cnts = vec![0; n];
//     let mut period_sum = 0;
//     let mut local_sum = 0;
//     for i in 0..n {
//         for j in 0..n {
//             if a[i] > a[j] {
//                 // period_cnts[i] += 1;
//                 period_sum += 1;
//                 if i < j {
//                     local_sum += 1;
//                 }
//             }
//         }
//     }
// 
//     let mut res = 0;
//     let unit;
//     if k % 2 == 0 {
//         unit = local_sum + period_sum * k / 2;
//     } else {
//         unit = local_sum + period_sum * (k - 1) / 2;
//     }
//     for i in 0..k {
//         if k % 2 == 0 && i == (k - 1) {
//             res += local_sum;
//         } else {
//             res += unit;
//         }
//         res %= MOD;
//     }
// 
//     println!("{}", res);
// }

// #[fastout]
// fn main() {
//     input! {
//         n: usize,  k: usize,
//         a: [usize; n]
//     }
// 
//     // let mut period_cnts = vec![0; n];
//     let mut period_sum = 0;
//     let mut local_sum = 0;
//     for i in 0..n {
//         for j in 0..n {
//             if a[i] > a[j] {
//                 // period_cnts[i] += 1;
//                 period_sum += 1;
//                 if i < j {
//                     local_sum += 1;
//                 }
//             }
//         }
//     }
// 
//     let mut res = 0;
//     let mut cum_period_sum = 0;
//     for _ in 0..k {
//         res = (res + local_sum + cum_period_sum) % MOD;
//         cum_period_sum = (cum_period_sum + period_sum) % MOD;
//     }
// 
//     println!("{}", res);
// }
