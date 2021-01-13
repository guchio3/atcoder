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
        h: usize, w: usize, _k: usize,
        s: [Chars; h]
    }
    let mut num = 1;
    let mut res = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if res[i][j] != 0 {
                continue;
            }
            let mut cake_cnt = 0;
            let mut k = i;
            let mut l = j;
            if s[k][l] == '#' {
                cake_cnt += 1;
            }
            'outer: while k < h - 1 || l < w - 1 {
                k = min(k + 1, h - 1);
                for ll in j..=l {
                    if s[k][ll] == '#' {
                        if cake_cnt == 0 {
                            cake_cnt += 1;
                        } else {
                            for ii in i..=k {
                                for jj in j..ll {
                                    res[ii][jj] = num;
                                }
                            }
                            num += 1;
                            break 'outer;
                        }
                    }
                }
                l = min(l + 1, w - 1);
                for kk in i..=k {
                    if s[kk][l] == '#' {
                        if cake_cnt == 0 {
                            cake_cnt += 1;
                        } else {
                            for ii in i..kk {
                                for jj in j..=l {
                                    res[ii][jj] = num;
                                }
                            }
                            num += 1;
                            break 'outer;
                        }
                    }
                }
            }
            if k == h - 1 && l == w - 1 && s[h - 1][w - 1] == '.' {
                for ii in i..h {
                    for jj in j..w {
                        res[ii][jj] = num;
                    }
                }
                num += 1;
            }
        }
    }
    for res_i in res {
        println!("{}", res_i.iter().join(" "));
    }
}
