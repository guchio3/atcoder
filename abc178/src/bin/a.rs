use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: usize
    }
    println!("{}", (x == 0) as usize);
}
