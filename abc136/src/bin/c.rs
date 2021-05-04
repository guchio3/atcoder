use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [i64; n]
    }
    h = h.into_iter().rev().collect();
    for i in 0..n {
        if i == 0 {
            continue;
        }
        if h[i] - h[i - 1] > 1 {
            println!("No");
            return;
        } else if h[i] - h[i - 1] == 1 {
            h[i] -= 1;
        }
    }
    println!("Yes");
}
