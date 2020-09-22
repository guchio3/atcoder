use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let mut res = 0;
    for (s_i, t_i) in s.chars().zip(t.chars()) {
        if s_i != t_i {
            res += 1;
        }
    }

    println!("{}", res);
}
