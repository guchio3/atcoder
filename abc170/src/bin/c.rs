use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: i32,
        n: usize,
        p: [i32; n]
    }

    let mut res = -1000;
    for i in 0..=100 {
        if !p.contains(&(x - i)) {
            res = x - i;
            break;
        } else if !p.contains(&(x + i)) {
            res = x + i;
            break;
        }
    }

    println!("{}", res);
}
