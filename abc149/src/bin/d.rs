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
        n: usize, k: usize,
        r: usize, s: usize, p: usize,
        t: Chars
    }
    let mut hand_dict = HashMap::new();
    hand_dict.insert(0, 'r');
    hand_dict.insert(1, 's');
    hand_dict.insert(2, 'p');
    let mut rev_hand_dict = HashMap::new();
    rev_hand_dict.insert('r', 0);
    rev_hand_dict.insert('s', 1);
    rev_hand_dict.insert('p', 2);

    let mut best_hands = vec![];
    for i in 0..n {
        let t_i = t[i];
        let best_hand;
        if t_i == 'r' {
            best_hand = 'p';
        } else if t_i == 's' {
            best_hand = 'r';
        } else {
            best_hand = 's';
        }
        best_hands.push(best_hand);
    }
    let mut res = 0;
    let mut used = vec![];
    for i in 0..n {
        let mut hand = best_hands[i];
        if i >= k {
            if used[i - k] == hand {
                if i > n - k - 1 || hand == best_hands[i + k] {
                    hand = *hand_dict
                        .get(&((rev_hand_dict.get(&hand).unwrap() + 1) % 3))
                        .unwrap();
                } else {
                    let mut next_hand_num = (rev_hand_dict.get(&hand).unwrap() + 1) % 3;
                    next_hand_num +=
                        (next_hand_num == *rev_hand_dict.get(&best_hands[i + k]).unwrap()) as usize;
                    next_hand_num %= 3;
                    hand = *hand_dict.get(&next_hand_num).unwrap();
                }
            }
        }
        if hand == best_hands[i] {
            if hand == 'r' {
                res += r;
            } else if hand == 'p' {
                res += p;
            } else {
                res += s;
            }
        }
        used.push(hand);
    }
    println!("{}", res);
}
