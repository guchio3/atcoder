use proconio::{input, fastout};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    let mut res = 1;
    let mut minus = 1;
    let mut minus_2 = 1;
    for _i in 0..n {
        res = res * 10 % MOD;
        minus = minus * 9 % MOD;
        minus_2 = minus_2 * 8 % MOD;
    }
    minus = (minus * 2 - minus_2) % MOD;

    if res - minus < 0 {
        res = MOD + (res - minus);
    } else {
        res = res - minus;
    }

    println!("{}", res);
}
