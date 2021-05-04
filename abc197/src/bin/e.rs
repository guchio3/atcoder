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
        xc: [(i64, i64); n]
    }
    let mut color_places = HashMap::new();
    color_places.insert(0, vec![0]);
    color_places.insert(n as i64 + 1, vec![0]);
    for i in 0..n {
        let xc_i = xc[i];
        (*color_places.entry(xc_i.1).or_insert(vec![])).push(xc_i.0);
    }
    let mut color_places: Vec<(i64, Vec<i64>)> =
        color_places.into_iter().map(|(k, v)| (k, v)).collect();
    color_places.sort();

    let mut bef_place_costs = vec![(0, 0)];
    for i in 1..color_places.len() {
        let (_color, mut places) = color_places[i].clone();
        places.sort();

        let mut left_cost_diff_min = 9223372036854775807i64;
        let mut right_cost_diff_min = 9223372036854775807i64;
        for k in 0..bef_place_costs.len() {
            let (bef_place, bef_cost) = bef_place_costs[k];
            left_cost_diff_min = min(
                left_cost_diff_min,
                bef_cost
                    + (places[places.len() - 1] - bef_place).abs()
                    + (places[0] - places[places.len() - 1]).abs(),
            );
            right_cost_diff_min = min(
                right_cost_diff_min,
                bef_cost
                    + (places[0] - bef_place).abs()
                    + (places[places.len() - 1] - places[0]).abs(),
            );
        }

        let mut tmp_color_places = HashMap::new();
        for j in 0..places.len() {
            let place = places[j];
            let cost_diff = min(
                left_cost_diff_min + (place - places[0]).abs(),
                right_cost_diff_min + (place - places[places.len() - 1]).abs(),
            );
            // println!("places[places.len() - 1]: {}", places[places.len() - 1]);
            // println!("places[0]: {}", places[0]);
            // println!("bef_place: {}", bef_place);
            // println!("place: {}", place);
            // println!("{}", cost_diff);
            // tmp_color_places.push((place, bef_cost + cost_diff));
            tmp_color_places.entry(place).or_insert(cost_diff);
            let tmp_value = tmp_color_places.get_mut(&place).unwrap();
            *tmp_value = min(*tmp_value, cost_diff);
            // println!("{:?}", tmp_color_places);
        }
        bef_place_costs = vec![];
        for (key, value) in tmp_color_places {
            bef_place_costs.push((key, value));
        }
        // bef_place_costs = tmp_color_places;
    }

    println!("{}", bef_place_costs[0].1);
}
