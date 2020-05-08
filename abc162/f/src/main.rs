use std::io::stdin;
use std::str::FromStr;
use std::cmp::max;

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

fn main() {
    // DP 使う問題っぽい
    let in_vec: Vec<i64> = read_line();
    let n = in_vec[0];
    let a: Vec<i64> = read_line();

    // 何回 +2 できるか
    let plus_2_num = 2 + (n % 2);

    let mut dp: Vec<Vec<Option<i64>>> = vec![vec![None;plus_2_num as usize];(n/2) as usize];

    for i in 0..n/2 {
        let i = i as usize;
        for j in 0..plus_2_num {
            let j = j as usize;
            if i == 0 {
                dp[i][j] = Some(a[2*i+j]);
            } else {
                let better_num;
                if j > 0 {
                    better_num = Some(max(dp[i-1][j].unwrap(), dp[i-1][j-1].unwrap()));
                } else {
                    better_num = dp[i-1][j];
                }
                dp[i][j] = Some(better_num.unwrap() + a[2*i+j]);
            }
        }
    }

    let mut res = dp[(n/2-1) as usize][0].unwrap();
    for i in 1..plus_2_num {
        res = max(res, dp[(n/2-1) as usize][i as usize].unwrap());
    }

    println!("{}", res);
}
