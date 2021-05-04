use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, x: usize,
        mut a: [usize; n]
    }
    let mut res = 0;
    for i in 0..n - 1 {
        if a[i] > x {
            res += a[i] - x;
            a[i] = x;
        }
        if a[i] + a[i + 1] > x {
            res += a[i] + a[i + 1] - x;
            a[i + 1] -= a[i] + a[i + 1] - x;
        }
    }
    println!("{}", res);
}
