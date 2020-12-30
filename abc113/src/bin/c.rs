#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, m: usize,
        mut py: [(usize, usize); m]
    }
    let mut new_py = vec![];
    for (i, &py_i) in py.iter().enumerate() {
        new_py.push((py_i.0, py_i.1, i));
    }
    let mut py = new_py;
    py.sort_by(|py1, py2| py1.1.cmp(&py2.1));
    let mut res = vec![];
    let mut city_cnts = HashMap::new();
    for py_i in py {
        *(city_cnts.entry(py_i.0).or_insert(0)) += 1;
        let mut x_cmp: Vec<char> = (*city_cnts.get(&py_i.0).unwrap())
            .to_string()
            .chars()
            .collect();
        let mut x = vec!['0'; 6 - x_cmp.len()];
        x.append(&mut x_cmp);
        let mut recog_num_cmp: Vec<char> = py_i.0.to_string().chars().collect();
        let mut recog_num = vec!['0'; 6 - recog_num_cmp.len()];
        recog_num.append(&mut recog_num_cmp);
        recog_num.append(&mut x);

        let res_i: String = recog_num.iter().collect();

        res.push((res_i, py_i.2));
    }

    res.sort_by(|py1, py2| py1.1.cmp(&py2.1));

    for res_i in res {
        println!("{}", res_i.0);
    }
}
