use proconio::{input, fastout};

fn main() {
    input! {
        a: usize, b: usize, c: usize
    }
    let mut used = vec![false; b];
    let mut i = 1;
    loop {
        let key = (a * i) % b;
        if used[key] {
            println!("NO");
            return;
        }
        if key == c {
            println!("YES");
            return;
        } else {
            used[key] = true;
            i += 1;
        }
    }
}
