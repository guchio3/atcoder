use proconio::input;
use std::cmp::{max, min};

fn main() {
    input!{
        n: usize,
        k: usize,
        p: [usize; n],
        c: [i64; n],
    }

    let min_c_value = *c.iter().min().unwrap();
    let mut res = min_c_value;

    // どこから始めるか
    for i in 0..n {
        let mut period = 0;
        let mut period_i = i;
        let mut period_sum = 0;
        while period == 0 || period_i != i {
            period += 1;
            period_sum += c[period_i];
            period_i = p[period_i] - 1;
        }

        let mut temp_res;
        if period_sum > 0 && k / period != 0 {
            temp_res = period_sum * ((k / period) as i64);
            if k % period == 0 {
                res = max(res, temp_res);
                continue;
            }
            let mut j = i + 1;
            let mut temp_temp_res = min_c_value;
            let mut temp_temp_sum = 0;
            for _j in 0..(k % period) {
                j = p[j - 1];
                // temp_temp_sum += c[p[j - 1] - 1];
                temp_temp_sum += c[j - 1];
                temp_temp_res = max(temp_temp_res, temp_temp_sum);
            }
            if temp_temp_res > 0 {
                temp_res += temp_temp_res;
            }
        } else {
            temp_res = min_c_value;
            let mut j = i + 1;
            let mut temp_sum = 0;
            for _j in 0..min(period, k) {
                j = p[j - 1];
                // temp_sum += c[p[j - 1] - 1];
                temp_sum += c[j - 1];
                temp_res = max(temp_res, temp_sum);
            }
        }

        res = max(res, temp_res);
    }

    println!("{}", res);
}
