use proconio::marker::Chars;
use proconio::{fastout, input};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }

    let mut dp = vec![vec![0; w]; h];
    let mut dp_h = vec![vec![0; w]; h];
    let mut dp_w = vec![vec![0; w]; h];
    let mut dp_d = vec![vec![0; w]; h];

    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            if i != 0 {
                dp_h[i][j] = (dp_h[i - 1][j] + dp[i - 1][j]) % MOD;
            }
            if j != 0 {
                dp_w[i][j] = (dp_w[i][j - 1] + dp[i][j - 1]) % MOD;
            }
            if i != 0 && j != 0 {
                dp_d[i][j] = (dp_d[i - 1][j - 1] + dp[i - 1][j - 1]) % MOD;
            }
            if !(i == 0 && j == 0) {
                dp[i][j] = (dp_h[i][j] + dp_w[i][j] + dp_d[i][j]) % MOD;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}

// #[fastout]
// fn main() {
//     input! {
//         h: usize, w: usize,
//         s: [Chars; h]
//     }
//
//     let mut dp = vec![vec![0; w]; h];
//     let mut used_h = vec![vec![false; w]; h];
//     let mut used_w = vec![vec![false; w]; h];
//     let mut used_d = vec![vec![false; w]; h];
//
//     dp[0][0] = 1;
//     for i in 0..h {
//         for j in 0..w {
//             println!("--------------------------");
//             println!("{:?}", used_w);
//             println!("{:?}", used_h);
//             println!("{:?}", used_d);
//             if s[i][j] == '#' {
//                 continue;
//             }
//
//             if !used_w[i][j] {
//                 used_w[i][j] = true;
//                 // 右
//                 for k in (j + 1)..w {
//                     if s[i][k] == '#' {
//                         break;
//                     }
//                     dp[i][k] = (dp[i][k] + dp[i][j] * (k - j)) % MOD;
//                     used_w[i][k] = true;
//                 }
//             }
//
//             if !used_h[i][j] {
//                 used_h[i][j] = true;
//                 // 下
//                 for k in (i + 1)..h {
//                     if s[k][j] == '#' {
//                         break;
//                     }
//                     dp[k][j] = (dp[k][j] + dp[i][j] * (k - i)) % MOD;
//                     used_h[k][j] = true;
//                 }
//             }
//
//             if !used_d[i][j] {
//                 used_d[i][j] = true;
//                 let mut k = i + 1;
//                 let mut l = j + 1;
//                 while k < h && l < w && s[k][l] != '#' {
//                     used_d[k][l] = true;
//                     dp[k][l] = (dp[k][l] + dp[i][j] * (k - i)) % MOD;
//                     k += 1;
//                     l += 1;
//                 }
//             }
//         }
//     }
//
//     println!("{:?}", dp);
//
//     println!("{}", dp[h - 1][w - 1]);
// }

// #[fastout]
// fn main() {
//     input! {
//         h: usize, w: usize,
//         s: [Chars; h]
//     }
//
//     let mut dp = vec![vec![0; w]; h];
//     dp[0][0] = 1;
//     for i in 0..h {
//         for j in 0..w {
//             if s[i][j] == '#' {
//                 continue;
//             }
//             // 右
//             // let mut k = j + 1;
//             for k in (j+1)..w {
//                 if s[i][k] == '#' {
//                     break;
//                 }
//                 dp[i][k] = (dp[i][k] + dp[i][j]) % MOD;
//             }
//             // while k < w && s[i][k] != '#' {
//             //     dp[i][k] = (dp[i][k] + dp[i][j]) % MOD;
//             //     k += 1;
//             // }
//
//             // 下
//             for k in (i+1)..h {
//                 if s[k][j] == '#' {
//                     break;
//                 }
//                 dp[k][j] = (dp[k][j] + dp[i][j]) % MOD;
//             }
//             // let mut k = i + 1;
//             // while k < h && s[k][j] != '#' {
//             //     dp[k][j] = (dp[k][j] + dp[i][j]) % MOD;
//             //     k += 1;
//             // }
//
//             // 右下
//             let mut k = i + 1;
//             let mut l = j + 1;
//             while k < h && l < w && s[k][l] != '#' {
//                 dp[k][l] = (dp[k][l] + dp[i][j]) % MOD;
//                 k += 1;
//                 l += 1;
//             }
//         }
//     }
//
//     println!("{}", dp[h - 1][w - 1]);
// }
