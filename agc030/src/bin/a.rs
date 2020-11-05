use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    let res;
    if a + b >= c {
        res = b + c;
    } else {
        res = b + (a + b) + 1;
    }
    println!("{}", res);
}
