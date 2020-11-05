use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }
    if s[s.len()-1] == 's' {
        s.push('e');
    }
    s.push('s');
    let res: String = s.iter().collect();
    println!("{}", res);
}
