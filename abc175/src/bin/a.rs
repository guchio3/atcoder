use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut res = 0;
    for s_i in s {
        if s_i == 'S' && res > 0 {
            break;
        } else if s_i == 'R' {
            res += 1;
        }
    }
    println!("{}", res);
}
