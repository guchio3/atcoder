use proconio::{fastout, input};
use std::collections::HashMap;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut cnt_map = HashMap::new();
    for a_i in a {
        *(cnt_map.entry(a_i - 1).or_insert(0)) += 1;
        *(cnt_map.entry(a_i).or_insert(0)) += 1;
        *(cnt_map.entry(a_i + 1).or_insert(0)) += 1;
    }

    let mut res = 0;
    for &v in cnt_map.values() {
        res = max(res, v);
    }

    println!("{}", res);
}
