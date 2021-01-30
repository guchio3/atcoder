#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::Into;
use std::fmt::Debug;

/// 1 start $B$J$3$H$K5$$r$D$1$k(B
struct BIT {
    n: usize,
    nodes: Vec<i64>,
    num_of_inversions: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT {
            n: n,
            nodes: vec![0; n + 1],
            num_of_inversions: vec![0; n],
        }
    }

    fn add(&mut self, mut i: usize, value: i64) {
        while i <= self.n {
            self.nodes[i] += value;
            i += (i as i64 & -(i as i64)) as usize;
        }
    }

    fn range_sum_from_start_to(&self, mut to: usize) -> i64 {
        let mut res = 0;
        while to > 0 {
            res += self.nodes[to];
            to -= (to as i64 & -(to as i64)) as usize;
        }
        res
    }
    fn range_sum_from_to(&self, from: usize, to: usize) -> i64 {
        self.range_sum_from_start_to(to) - self.range_sum_from_start_to(from)
    }

    fn calc_num_of_inversions(&mut self, nums: &Vec<i64>) -> i64 {
        let mut res = 0;
        for i in 0..nums.len() {
            let num = nums[i] as usize;
            let num_of_inversions = i as i64 - self.range_sum_from_start_to(num);
            self.num_of_inversions.push(num_of_inversions);
            res += num_of_inversions;
            self.add(num, 1);
        }
        res
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let a: Vec<i64> = a.into_iter().map(|x| x + 1).collect();
    let mut bit = BIT::new(n);
    let mut num_of_inversions = bit.calc_num_of_inversions(&a);
    println!("{}", num_of_inversions);
    for i in 0..n - 1 {
        num_of_inversions += n as i64 - a[i];
        num_of_inversions -= a[i] - 1;
        println!("{}", num_of_inversions);
    }
}

// fn main() {
//     input! {
//         n: usize,
//         a: [i64; n]
//     }
//     let mut a: VecDeque<i64> = a.into_iter().map(|x| x + 1).collect();
//     for _i in 0..n {
//         let mut bit = BIT::new(n);
//         let num_of_inversions = bit.calc_num_of_inversions(&a);
//         let last = a.pop_front().unwrap();
//         a.push_back(last);
//         println!("{}", num_of_inversions);
//     }
// }
