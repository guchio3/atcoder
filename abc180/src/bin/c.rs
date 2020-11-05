use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut res = vec![];
    let mut rev_res = vec![];
    let root_n = (n as f64).sqrt() as usize;
    for n_i in 1..=root_n {
        if n % n_i == 0 {
            res.push(n_i);
            if n_i != (n / n_i) {
                rev_res.push(n / n_i);
            }
        }
    }
    for &rev_res_i in rev_res.iter().rev() {
        res.push(rev_res_i);
    }
    for res_i in res {
        println!("{}", res_i);
    }
}
