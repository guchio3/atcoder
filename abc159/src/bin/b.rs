use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    for i in 0..n/2 {
        if s[i] != s[s.len()-i-1]{
            println!("No");
            return;
        }
    }
    for i in 0..((n/2)/2) {
        if s[i] != s[n/2-i-1]{
            println!("No");
            return;
        }
    }
    for i in (n+3)/2..((n+3)/2+n)/2 {
        if s[i] != s[n-i-1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
