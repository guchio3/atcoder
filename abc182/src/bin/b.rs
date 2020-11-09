use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut res = 0;
    let mut max_res = 0;
    for i in 2..=1000 {
        let mut res_i = 0;
        for &a_i in a.iter() {
            if a_i % i == 0 {
                res_i += 1;
            }
        }
        if max_res <= res_i {
            max_res = res_i;
            res = i;
        }
    }

    println!("{}", res);
}
