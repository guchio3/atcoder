#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut cnts = vec![vec![]; 200];

    for i in 0..n {
        a[i] %= 200;
        let mut elem = HashSet::new();
        elem.insert(i);
        cnts[a[i]].push(elem);

        for j in 0..200 {
            for k in 0..cnts[j].len() {
                let mut next_set = cnts[j][k].clone();
                next_set.insert(i);
                let next_num = next_set.iter().map(|x| a[*x]).sum::<usize>() % 200;
                if cnts[next_num].len() > 0 {
                    for l in 0..cnts[next_num].len() {
                        let next_num_set = cnts[next_num][l].clone();
                        let intersection_set: HashSet<usize> =
                            next_set.intersection(&next_num_set).map(|x| *x).collect();
                        if intersection_set.len() == 0 {
                            let mut res_next_set: Vec<usize> =
                                next_set.clone().into_iter().map(|x| x + 1).collect();
                            res_next_set.sort();
                            let mut res_next_num_vec: Vec<usize> =
                                next_num_set.clone().into_iter().map(|x| x + 1).collect();
                            res_next_num_vec.sort();
                            println!("Yes");
                            println!(
                                "{} {}",
                                res_next_set.len(),
                                res_next_set.clone().into_iter().join(" ")
                            );
                            println!(
                                "{} {}",
                                res_next_num_vec.len(),
                                res_next_num_vec.clone().into_iter().join(" ")
                            );
                            return;
                        }
                    }
                }
                cnts[next_num].push(next_set);
            }
        }
    }

    println!("No");
}
