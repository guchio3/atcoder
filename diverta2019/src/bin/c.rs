#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut res = 0;
    let mut start_b_cnt = 0;
    let mut end_a_cnt = 0;
    let mut start_b_and_end_a = 0;
    for s_i in s {
        if s_i[0] == 'B' {
            start_b_cnt += 1;
            if s_i[s_i.len() - 1] == 'A' {
                start_b_and_end_a += 1;
            }
        }
        if s_i[s_i.len() - 1] == 'A' {
            end_a_cnt += 1;
        }
        let mut bef_char = ' ';
        for s_i_j in s_i {
            if bef_char == 'A' && s_i_j == 'B' {
                res += 1;
            }
            bef_char = s_i_j;
        }
    }

    // all zero をケアしないとダメ
    if start_b_cnt != 0 || end_a_cnt != 0 {
        res += min(start_b_cnt, end_a_cnt)
            - (max(start_b_cnt, end_a_cnt) == start_b_and_end_a) as usize;
    }

    println!("{}", res);
}
