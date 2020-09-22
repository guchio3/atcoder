use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        bc: [(usize, usize); q]
    }
    let mut temp_sum = 0;
    let mut elem_dict = HashMap::new();
    for &a_i in a.iter() {
        temp_sum += a_i;
        (*elem_dict.entry(a_i).or_insert(0)) += 1;
    }

    let mut s = Vec::<usize>::new();
    for (b_i, c_i) in bc {
        if let Some(&i) = elem_dict.get(&b_i) {
            // ↓ だと usize がマイナスになり得るので panic する
            // temp_sum += (c_i - b_i) * i;
            temp_sum -= b_i * i;
            temp_sum += c_i * i;
            (*elem_dict.entry(c_i).or_insert(0)) += i;
            elem_dict.insert(b_i, 0);
        }
        s.push(temp_sum);
    }

    for s_i in s {
        println!("{}", s_i);
    }
}
