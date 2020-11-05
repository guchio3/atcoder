use proconio::{input, fastout};

const MOD: usize = 1_000_000_007;

// fn dfs(temp_sum: usize, mut res: usize, s: usize) -> usize {
//     if temp_sum > s {
//         res % MOD
//     } else if temp_sum == s {
//         (res + 1) % MOD
//     } else {
//         for i in 3..=9 {
//             res = dfs(temp_sum + i, res, s);
//         }
//         res % MOD
//     }
// }
// 
// #[fastout]
// fn main() {
//     input! {
//         s: usize,
//     }
//     let res = 0;
//     println!("{}", dfs(0, res, s));
// }

#[fastout]
fn main() {
    input! {
        s: usize,
    }
    let mut dps = vec![0; s];
    for i in 2..s {
        dps[i] = 1;
    }

    for i in 0..s {
        if i < 3 {
            continue;
        }
        let mut dpi = 0;
        for j in 0..=i-3 {
            dpi = (dpi + dps[j]) % MOD;
        }
        dps[i] += dpi;
    }
    println!("{}", dps[s-1]);
}
