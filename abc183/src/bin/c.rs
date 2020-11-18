use proconio::{input, fastout};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        t: [[usize; n]; n]
    }

    let mut res = 0;
    for perm_vec in (0..n).permutations(n) {
        let mut cost = 0;
        let mut last_elem = 0;
        for perm_vec_i in perm_vec {
            cost += t[last_elem][perm_vec_i];
            last_elem = perm_vec_i;
        }
        if last_elem == 0 && cost == k {
            res += 1;
        }
    }

    println!("{}", res);
}


// fn dfs(now: usize, t: &Vec<Vec<usize>>, used: &mut Vec<bool>, k: usize, tmp_cost: usize) -> usize {
//     let mut tmp_res = 0;
//     let mut used_cnt = 0;
//     for i in 0..used.len() {
//         if !used[i] {
//             used[i] = true;
//             tmp_res += dfs(i, t, used, k, tmp_cost + t[now][i]);
//             used[i] = false;
//         } else {
//             used_cnt += 1;
//         }
//     }
// 
//     if used_cnt == used.len() {
//         if tmp_cost == k && now == 0 {
//             return 1;
//         } else {
//             return 0;
//         }
//     }
// 
//     tmp_res
// }
// 
// #[fastout]
// fn main() {
//     input! {
//         n: usize, k: usize,
//         t: [[usize; n]; n]
//     }
// 
//     let mut used = vec![false; n];
//     let res = dfs(0, &t, &mut used, k, 0);
// 
//     println!("{}", res);
// }
