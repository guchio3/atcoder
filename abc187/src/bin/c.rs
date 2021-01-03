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
        n: usize,
        s: [Chars; n]
    }
    let mut ex = HashSet::new();
    let mut non_ex = HashSet::new();
    for s_i in s {
        if s_i[0] == '!' {
            ex.insert(s_i);
        } else {
            non_ex.insert(s_i);
        }
    }
    for mut non_ex_i in non_ex {
        let mut non_ex_vec = vec!['!'];
        let res: String = non_ex_i.iter().join("");
        non_ex_vec.append(&mut non_ex_i);
        if ex.contains(&non_ex_vec) {
            // let res: String = non_ex_vec.iter().join("");
            println!("{}", res);
            return;
        }
    }

    println!("satisfiable");
}
