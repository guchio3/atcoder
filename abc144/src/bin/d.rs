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
        a: f64, b: f64, x: f64,
    }
    let bottom = a * a;
    let all = bottom * b;
    if all >= x * 2. {
        let tri_area = x / a;
        let res = (tri_area * 2. / b / b).atan();
        println!("{}", 90. - 180. * res / std::f64::consts::PI);
    } else {
        let b_diff = (all - x) * 2. / (a * a);
        let res = (a / b_diff).atan();
        println!("{}", 90. - 180. * res / std::f64::consts::PI);
        // println!(
        //     "{}",
        //     (2. * (a * a * b - x) / (a * a * a)).atan() * 180. / std::f64::consts::PI
        // );
    }
}
