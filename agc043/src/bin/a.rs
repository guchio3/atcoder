use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::min;

const LARGE_NUM: usize = 1_000_000_000;

fn dfs(
    s: &Vec<Vec<char>>,
    min_costs: &mut Vec<Vec<usize>>,
    point: (usize, usize),
    point_lim: (usize, usize),
    bef_char: char,
    mut tmp_res: usize,
) -> usize {
    let h_i = point.0;
    let w_i = point.1;
    let h_lim = point_lim.0;
    let w_lim = point_lim.1;

    if h_i >= h_lim || w_i >= w_lim {
        return LARGE_NUM;
    }

    if min_costs[h_i][w_i] <= tmp_res {
        return LARGE_NUM;
    }

    let curr_char = s[h_i][w_i];
    if bef_char == '#' && curr_char == '.' {
        tmp_res += 1;
    }
    min_costs[h_i][w_i] = tmp_res;

    if h_i == (h_lim - 1) && w_i == (w_lim - 1) {
        if curr_char == '#' {
            tmp_res += 1;
        }
        return tmp_res;
    }

    let mut res = LARGE_NUM; // large num
    if h_i < h_lim {
        res = min(
            res,
            dfs(s, min_costs, (h_i + 1, w_i), point_lim, curr_char, tmp_res),
        );
    }
    if w_i < w_lim {
        res = min(
            res,
            dfs(s, min_costs, (h_i, w_i + 1), point_lim, curr_char, tmp_res),
        );
    }

    res
}

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }
    let mut min_costs = vec![vec![LARGE_NUM; w]; h];
    let res = dfs(&s, &mut min_costs, (0, 0), (h, w), s[0][0], 0);
    println!("{}", res);
}
