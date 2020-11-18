use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: i64,
    }

    let res;
    if x >= 0 {
        res = x;
    } else {
        res = 0;
    }

    println!("{}", res);
}
