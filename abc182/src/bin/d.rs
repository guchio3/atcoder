use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut cumcumsums = vec![];
    let mut maxes = vec![];

    let mut temp_cumsum = 0;
    let mut temp_cumcumsum = 0;
    let mut temp_max = 0;
    for a_i in a.iter() {
        temp_cumsum += a_i;
        temp_cumcumsum += temp_cumsum;
        temp_max = max(temp_max, temp_cumsum);
        cumcumsums.push(temp_cumcumsum);
        maxes.push(temp_max);
    }

    let mut res = 0;
    let mut before_res = 0;
    for i in 0..n {
        res = max(before_res + maxes[i], res);
        before_res = cumcumsums[i];
    }

    println!("{}", res);
}
