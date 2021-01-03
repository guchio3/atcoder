fn ncr(n: usize, mut r: usize) -> usize {
    if r > n / 2 {
        r = n - r;
    }
    let mut res = 1;
    let start = max(n - r + 1, r);
    let mut r_vec: Vec<usize> = (1..=r).into_iter().collect();
    for i in start..=n {
        res *= i;
        let mut non_used = vec![];
        for &r_vec_i in r_vec.iter() {
            if res % r_vec_i == 0 {
                res /= r_vec_i;
            } else {
                non_used.push(r_vec_i);
            }
        }
        r_vec = non_used;
    }
    res
}
