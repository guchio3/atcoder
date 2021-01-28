use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [usize; n]
    }
    let mut i = 0;
    let mut res = 0;
    while i < n {
        if p[i] == i + 1 {
            i += 1;
            res += 1;
        }
        i += 1;
    }
    println!("{}", res);
}
