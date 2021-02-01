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
