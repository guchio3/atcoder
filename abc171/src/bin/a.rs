use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        alpha: char,
    }

    // println!("a: {}, A: {}", 'a' as i32, 'A' as i32);
    let res = if (alpha as i32) >= ('a' as i32) {'a'} else {'A'};

    println!("{}", res);
}
