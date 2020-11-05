use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut candidates = vec![];
    for i in 0..1000 {
        if i % 8 == 0 {
            let str_num: String = i.to_string();
            if i < 10 {
                candidates.push(vec!['0', '0', str_num[0]]);
            } else if i < 100 {
                ;
            } else {
                candidates.push();
            }
        }
    }
}
