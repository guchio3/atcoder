use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n]
    }
    let mut cnt = 0;
    for (d_1, d_2) in d {
        if d_1 == d_2 {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt > 2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
