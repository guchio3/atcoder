use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        l: f64
    }
    println!("{}", (l/3.).powi(3));
}
