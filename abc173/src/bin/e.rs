use proconio::{input, fastout};

const MOD_UNIT: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }

    a.sort_by(|i, j| i.abs().cmp(j).reverse());

    let mut negative_cnt = 0;
    let mut last_positive_i: Option<usize> = None;
    let mut last_negative_i: Option<usize> = None;
    let mut outer_first_positive_i: Option<usize> = None;
    let mut outer_first_negative_i: Option<usize> = None;
    for i in 0..(a.len()) {
        if i < k {
            if a[i] < 0 {
                negative_cnt += 1;
                last_negative_i = Some(i);
            } else if a[i] > 0 {
                last_positive_i = Some(i);
            } else {
                last_negative_i = Some(0);
                last_positive_i = Some(0);
            }
        } else {
            if negative_cnt % 2 == 0 {
                break;
            }
            if a[i] < 0 {
                outer_first_negative_i = Some(i);
                if outer_first_positive_i != None {
                    break;
                }
            } else {
                outer_first_positive_i = Some(i);
                if outer_first_negative_i != None {
                    break;
                }
            }
        }
    }

    let mut res = 1;
    if negative_cnt % 2 == 0 || a.len() == k {
        for i in 0..k {
            res = (res * a[i]) % MOD_UNIT;
            if res < 0 {
                res = MOD_UNIT + res;
            }
        }
    } else if last_positive_i == None {
        if outer_first_positive_i == None {
            for i in 0..k {
                res = (res * a[a.len() - i - 1]) % MOD_UNIT;
                if res < 0 {
                    res = MOD_UNIT + res;
                }
            }
        } else {
            for i in 0..k {
                if i == last_negative_i.unwrap() {
                    continue;
                }
                res = (res * a[i]) % MOD_UNIT;
                if res < 0 {
                    res = MOD_UNIT + res;
                }
            }
            res = (res * a[outer_first_positive_i.unwrap()]) % MOD_UNIT;
            if res < 0 {
                res = MOD_UNIT + res;
            }
        }
    } else if last_negative_i == None {
        panic!("can not happen");
    } else {
        if outer_first_positive_i == None 
            || a[last_positive_i.unwrap()] * a[outer_first_positive_i.unwrap()] 
                < a[last_negative_i.unwrap()] * a[outer_first_negative_i.unwrap()] {
            for i in 0..k {
                if i == last_positive_i.unwrap() {
                    continue;
                }
                res = (res * a[i]) % MOD_UNIT;
                if res < 0 {
                    res = MOD_UNIT + res;
                }
            }
            res = (res * a[outer_first_negative_i.unwrap()]) % MOD_UNIT;
            if res < 0 {
                res = MOD_UNIT + res;
            }
        } else {
            for i in 0..k {
                if i == last_negative_i.unwrap() {
                    continue;
                }
                res = (res * a[i]) % MOD_UNIT;
                if res < 0 {
                    res = MOD_UNIT + res;
                }
            }
            res = (res * a[outer_first_positive_i.unwrap()]) % MOD_UNIT;
            if res < 0 {
                res = MOD_UNIT + res;
            }
        }
    }

    // if res < 0 {
    //     res = MOD_UNIT + res;
    // }

    println!("{}", res);
}
