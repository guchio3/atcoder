use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut res = 0;
    for (a_i, b_i) in ab {
        res += (a_i + b_i) * (b_i - a_i + 1) / 2;
    }
    println!("{}", res);
}
