use proconio::{input};
use std::collections::HashSet;

// #[fastout]
fn main() {
    input! {
        k: usize
    }

    let mut res = 0;
    // let mut used_pair: Vec<Vec<bool>> = vec![vec![false; k]; k];
    let mut used_pair = HashSet::<(usize, usize)>::new();

    let mut former_mod_res = 0;
    let mut mod_frag = 7 % k;
    loop {
        res += 1;
        // if used_pair[former_mod_res][mod_frag] {
        if used_pair.contains(&(former_mod_res, mod_frag)) {
            println!("-1");
            return;
        } else {
            // used_pair[former_mod_res][mod_frag] = true;
            used_pair.insert((former_mod_res, mod_frag));
        }
        former_mod_res = (former_mod_res + mod_frag) % k;
        if former_mod_res == 0 {
            println!("{}", res);
            return;
        } else {
            mod_frag = (mod_frag * 10) % k;
        }
    }
}
