use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut x: usize, m: usize
    }
    let mut res = x;
    let mut used_num = vec![(1_000_000, 0); 1_000_000];
    used_num[x] = (0, x);
    for i in 1..n {
        x = (x * x) % m;
        res += x;
        if used_num[x].0 != 1_000_000 {
            res = (res - used_num[x].1) * ((n - 1 - used_num[x].0) / (i - used_num[x].0))
                + used_num[x].1;
            for _ in 0..(n - 1 - used_num[x].0) % (i - used_num[x].0) {
                x = (x * x) % m;
                res += x;
            }
            break;
        } else {
            used_num[x] = (i, res);
        }
    }

    println!("{}", res);
}
