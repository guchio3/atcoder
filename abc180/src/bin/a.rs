use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }
    println!("{}", n+b-a);
}
