#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    // $B$G$-$k$@$1<h$j=|$+$J$$$[$&$,NI$$(B
    // $B$G$-$k$@$1=EJ#$5$;$?$[$&$,NI$$(B
    // $B"-(B
    // $B>.$5$$J}$K9g$o$;$k7A$G=EJ#$r$G$-$k$@$1$5$;$kJ}?K(B
    let mut diff_num = max(n, m) - min(n, m);
    
    let target_vec;
    if n > m {
        target_vec = b;
    } else {
        target_vec = a;
    }
    let mut best_non_dup_num = min(n, m);
    for i in 0..max(n, m) {
        for j in diff_num - (max(n, m) - 1 - i)..diff_num {
            ;
        }
    }
    println!("{}", diff_num + best_non_dup_num);
}
