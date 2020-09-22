use proconio::{input, fastout};
use std::cmp::{max};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut temp_cost = 0;
    let mut i = 0;
    while i < a.len() {
        if k < (temp_cost + a[i]) {
            break;
        }
        temp_cost += a[i];
        i += 1;
    }
    // println!("{}", i);
    let mut res = i;
    let mut j = 0;
    // while j < b.len() && k >= (temp_cost - a[i] + b[j]) {
    while j < b.len() {
        // println!("{}", j);
        if k >= temp_cost + b[j] {
            // println!("A");
            temp_cost += b[j];
            j += 1;
            res = max(i + j, res);
        } else {
            // println!("B");
            if i == 0 {
                break;
            }
            // println!("C -- {}", i);
            temp_cost -= a[i - 1];
            i -= 1;
        }
    }
    println!("{}", res);
}
