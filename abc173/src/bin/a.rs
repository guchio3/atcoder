use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", if n % 1_000 == 0 { 0 } else {1_000 - n % 1_000});
}
