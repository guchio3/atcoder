use proconio::marker::Chars;
use proconio::{fastout, input};

fn iter_cnt(target: char, s: &Vec<char>) -> Vec<i32> {
    let mut left_ptr = 0;
    let mut right_ptr = 0;
    let mut cnt = vec![0; s.len()];
    while right_ptr < s.len() - 1 {
        right_ptr += 1;
        if s[right_ptr] == target {
            if (right_ptr - left_ptr) % 2 == 0 {
                cnt[left_ptr] += 1;
            } else {
                cnt[left_ptr + 1] += 1;
            }
        } else {
            left_ptr = right_ptr;
        }
    }
    cnt
}

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let s_len = s.len();
    let mut res = vec![0; s_len];

    let res_l = iter_cnt('L', &s);
    let rev_s = &s.into_iter().rev().collect();
    let res_r = iter_cnt('R', &rev_s);
    let res_r: Vec<i32> = res_r.into_iter().rev().collect();
    // println!("{:?}", res_l);
    // println!("{:?}", res_r);

    for i in 0..s_len {
        res[i] += res_l[i] + res_r[i];
    }

    // let res: Vec<String> = res.iter().map(|res_i| char::from_digit(*res_i as u32, 10).unwrap()).collect();
    let res: Vec<String> = res.iter().map(|res_i| res_i.to_string()).collect();
    println!("{}", res.join(" "));
}
