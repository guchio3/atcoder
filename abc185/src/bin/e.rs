#![allow(unused_imports)]
use itertools::Itertools;
use num;
use num::traits::{FromPrimitive, NumAssign, NumCast, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::From;

fn calc_levenshtein_distance<S, T>(chars1: Vec<S>, chars2: Vec<S>) -> T
where
    S: Eq,
    T: PrimInt,
{
    let len1 = chars1.len();
    let len2 = chars2.len();
    let mut dp = vec![vec![T::zero(); len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        dp[i][0] = T::from(i).unwrap();
    }
    for i in 0..=len2 {
        dp[0][i] = T::from(i).unwrap();
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
            dp[i][j] = dp[i - 1][j - 1] + T::from((chars1[i - 1] != chars2[j - 1]) as u8).unwrap();
            dp[i][j] = min(dp[i][j], dp[i][j - 1] + T::one());
            dp[i][j] = min(dp[i][j], dp[i - 1][j] + T::one());
        }
    }
    dp[len1][len2]
}

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
    // $B"-(B
    // $BJT=85wN%(B
    let res: usize = calc_levenshtein_distance(a, b);

    println!("{}", res);
}
