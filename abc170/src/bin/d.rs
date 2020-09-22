use proconio::{input};
// use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut devide_array: [usize; 1_000_005] = [0; 1_000_005];

    a.sort();

    for &a_i in &a {
        if devide_array[a_i] != 0 {
            devide_array[a_i] = 2;
            continue;
        }
        for j in 1..=(1_000_005/a_i) {
            if j * a_i >= 1_000_005 {
                continue;
            }
            devide_array[j * a_i] += 1;
        }
    }

    let mut res = 0;

    for a_i in a {
        if devide_array[a_i] == 1 {
            res += 1;
        }
    }

    // let mut invalid_ids = HashSet::new();
    // 'outer: for i in 0..a.len() {
    //     for j in 0..a.len() {
    //         if i <= j && a[i] != a[j] {
    //             break;
    //         } else if (i != j) && (a[i] % a[j] == 0) {
    //             invalid_ids.insert(i);
    //             continue 'outer;
    //         } else if invalid_ids.contains(&j) {
    //             continue;
    //         }
    //     }
    //     res += 1;
    // }

    println!("{}", res);
}
