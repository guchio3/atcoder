#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }
    let mut hors = vec![vec![0; w]; h];
    let mut virs = vec![vec![0; w]; h];
    for i in 0..h {
        let mut cums = 0;
        let mut start = 0;
        for j in 0..w {
            if s[i][j] == '#' {
                for k in start..j {
                    hors[i][k] = cums;
                }
                start = j + 1;
                cums = 0;
            } else {
                cums += 1;
            }
        }
        for k in start..w {
            hors[i][k] = cums;
        }
    }

    for i in 0..w {
        let mut cums = 0;
        let mut start = 0;
        for j in 0..h {
            if s[j][i] == '#' {
                for k in start..j {
                    virs[k][i] = cums;
                }
                start = j + 1;
                cums = 0;
            } else {
                cums += 1;
            }
        }
        for k in start..h {
            virs[k][i] = cums;
        }
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            res = max(res, virs[i][j] + hors[i][j]);
        }
    }
    if res > 0 {
        res -= 1;
    }
    println!("{}", res);
}
