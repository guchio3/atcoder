use proconio::{fastout, input};
use std::collections::HashSet;

fn money_break(a: &Vec<usize>, mut y: usize) -> HashSet<usize> {
    let mut used = HashSet::new();
    for &a_i in a.iter().rev() {
        while a_i <= y {
            used.insert(a_i);
            y -= a_i;
        }
    }
    used
}

fn dfs(
    a: &Vec<usize>,
    i: usize,
    mut used_money: HashSet<usize>,
    mut bef_y: usize,
    mut res: usize,
) -> usize {
    if i == a.len() {
        return res;
    }
    let a_i = a[i];
    let mut diff = 0;
    let mut before_temp_used_money = HashSet::new();
    while diff != a_i {
        bef_y += a_i;
        let temp_used_money = money_break(&a, bef_y);
        if used_money.is_disjoint(&temp_used_money) {
            res += 1;
        } else if temp_used_money == before_temp_used_money {
            break;
        }
        before_temp_used_money = temp_used_money;
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize, x: usize,
        a: [usize; n],
    }

    let res = dfs();

    println!("{}", res);
}
