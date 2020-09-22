use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: isize,
        d: isize,
        xy: [(isize, isize); n]
    }

    let mut res = 0;
    let dd = d.pow(2);

    for (x_i, y_i) in xy {
        if dd >= (x_i.pow(2) + y_i.pow(2)) {
            res += 1;
        }
    }

    println!("{}", res);
}
