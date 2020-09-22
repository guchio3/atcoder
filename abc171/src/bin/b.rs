use proconio::{input, fastout};

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    }

    p.sort();

    // println!("{}", p.truncate(k).iter());
    println!("{}", &p[..k].to_vec().iter().fold(0, |a, b| a+b));
}
