use proconio::{input, fastout};

const MOD: usize = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(usize, usize); k],
    }
    // dp & cumsum
    let mut dp = vec![0; n+1];
    let mut cumsums = vec![0; n+1];
    dp[1] = 1;
    cumsums[1] = 1;
    for i in 2..=n {
        for &(l_i, mut r_i) in lr.iter() {
            r_i += 1;
            if i < l_i {
                continue
            } else if i < r_i {
                r_i = i;
            }
            dp[i] = (dp[i] + cumsums[i - l_i] - cumsums[i - r_i]) % MOD;
        }
        cumsums[i] = dp[i] + cumsums[i-1];
    }

    println!("{}", dp[n]);
}


// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         k: usize,
//         lr: [(usize, usize); k],
//     }
//     let mut s = Vec::new();
//     for (l_i, r_i) in lr {
//         for i in l_i..=r_i {
//             s.push(i);
//         }
//     }
// 
//     // dp
//     let mut dp: Vec<usize> = vec![0; n];
//     dp[0] = 1;
//     for i in 1..n {
//         for &s_i in s.iter() {
//             if s_i <= i {
//                 dp[i] = (dp[i] + dp[i - s_i]) % MOD;
//             }
//         }
//     }
// 
//     println!("{}", dp[n-1]);
// }
