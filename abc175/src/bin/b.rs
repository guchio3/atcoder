use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }
    l.sort();

    let mut res = 0;

    for i in 0..(l.len()) {
        for j in (i+1)..(l.len()) {
            for k in (j+1)..(l.len()) {
                if l[i] != l[j] && l[j] != l[k] {
                    if l[k] < l[i] + l[j] {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}

