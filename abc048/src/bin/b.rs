use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, x: usize
    }

    let mut res = b / x + 1;
    if a > 0 {
        res -= a / x + 1;
        if a % x == 0 {
            res += 1;
        }
    }

    println!("{}", res);
}
