use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for a in 1..=n {
        res += n / a;
        if n % a == 0 {
            res -= 1;
        }
    }
    println!("{}", res);
}
