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

// from abc156_d
fn mod_pow(mut x: usize, mut n: usize, mod_num: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n & 1 > 0 {
            res = (res * x) % mod_num;
        }
        x = (x * x) % mod_num;
        n >>= 1;
    }
    res
}

// from abc156_d
fn mod_ncr(n: usize, mut r: usize, mod_num: usize) -> usize {
    if r > n / 2 {
        r = n - r;
    }
    let mut res = 1;
    for i in n - r + 1..=n {
        res = (res * i) % mod_num;
    }
    let mut rev_mod = 1;
    for i in 1..=r {
        rev_mod = (rev_mod * i) % mod_num;
    }
    rev_mod = mod_pow(rev_mod, mod_num - 2, mod_num);
    res = (res * rev_mod) % mod_num;
    res
}
