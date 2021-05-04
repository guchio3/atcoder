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
        x0: f64, y0: f64,
        xhalfn: f64, yhalfn: f64
    }
    let radius = ((xhalfn - x0).powi(2) + (yhalfn - y0).powi(2)).sqrt() / 2.;
    let rad = 2. * std::f64::consts::PI / (n as f64);
    let to_half = radius - radius * rad.cos();
    let vir_to_half = radius * rad.sin();
    let unit_vec = ((xhalfn - x0) / (2. * radius), (yhalfn - y0) / (2. * radius));
    let vir_unit_vec = (
        (yhalfn - y0) / (2. * radius),
        -1. * (xhalfn - x0) / (2. * radius),
    );
    let unit_vec_diff = (unit_vec.0 * to_half, unit_vec.1 * to_half);
    let vir_unit_vec_diff = (vir_unit_vec.0 * vir_to_half, vir_unit_vec.1 * vir_to_half);
    println!(
        "{} {}",
        x0 + unit_vec_diff.0 + vir_unit_vec_diff.0,
        y0 + unit_vec_diff.1 + vir_unit_vec_diff.1
    );
}
