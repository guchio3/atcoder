use num::traits::PrimInt;
use std::cmp::min;

fn levenshtein_distance<S, T>(chars1: Vec<S>, chars2: Vec<S>) -> T
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
