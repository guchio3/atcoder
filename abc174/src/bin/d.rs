use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        _n: usize,
        mut c: Chars,
    }

    let mut left_idx = 0;
    let mut right_idx = c.len() - 1;

    let mut res = 0;
    loop {
        while c[left_idx] == 'R' && left_idx < right_idx {
            left_idx += 1;
        }
        while c[right_idx] == 'W' && left_idx < right_idx {
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            break;
        } else {
            res += 1;
            left_idx += 1;
            right_idx -= 1;
        }
    }

    println!("{}", res);
}
