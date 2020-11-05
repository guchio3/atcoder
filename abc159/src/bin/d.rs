use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut cnt_dict = HashMap::<usize, usize>::new();
    for &a_i in a.iter() {
        *(cnt_dict.entry(a_i).or_insert(0)) += 1;
    }
    let sum =
        cnt_dict
            .values()
            .into_iter()
            .fold(0, |sum, &x| if x > 1 { sum + x * (x - 1) / 2 } else { sum });
    for a_i in a.iter() {
        let value = *cnt_dict.get(a_i).unwrap();
        if value > 2 {
            println!("{}", sum - value * (value - 1) / 2 + (value - 1) * (value - 2) / 2);
        } else if value > 1 {
            println!("{}", sum - value * (value - 1) / 2);
        } else {
            println!("{}", sum);
        }
    }
}
