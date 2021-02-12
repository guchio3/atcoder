#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }
    let white_nums = s
        .iter()
        .map(|s_i| s_i.iter().map(|s_ij| (*s_ij == '.') as i64).sum::<i64>())
        .sum::<i64>();
    let mut deque = VecDeque::new();
    let mut reses = vec![vec![-1; w]; h];
    reses[0][0] = 1;
    let mut used = vec![vec![false; w]; h];
    deque.push_back((0, 0));
    while deque.len() > 0 {
        let (i, j) = deque.pop_front().unwrap();
        if i == h - 1 && j == w - 1 {
            break;
        }
        if 0 < i && !used[i - 1][j] && s[i - 1][j] == '.' {
            deque.push_back((i - 1, j));
            used[i - 1][j] = true;
            reses[i - 1][j] = reses[i][j] + 1;
        }
        if 0 < j && !used[i][j - 1] && s[i][j - 1] == '.' {
            deque.push_back((i, j - 1));
            used[i][j - 1] = true;
            reses[i][j - 1] = reses[i][j] + 1;
        }
        if i < h - 1 && !used[i + 1][j] && s[i + 1][j] == '.' {
            deque.push_back((i + 1, j));
            used[i + 1][j] = true;
            reses[i + 1][j] = reses[i][j] + 1;
        }
        if j < w - 1 && !used[i][j + 1] && s[i][j + 1] == '.' {
            deque.push_back((i, j + 1));
            used[i][j + 1] = true;
            reses[i][j + 1] = reses[i][j] + 1;
        }
    }
    if reses[h - 1][w - 1] == -1 {
        println!("-1");
    } else {
        println!("{}", white_nums - reses[h - 1][w - 1]);
    }
}
