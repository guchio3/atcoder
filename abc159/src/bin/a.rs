use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }
    let n_res;
    let m_res;
    if n > 1 {
        n_res = n*(n-1)/2;
    } else {
        n_res = 0;
    }
    if m > 1 {
        m_res = m*(m-1)/2;
    } else {
        m_res = 0;
    }
    println!("{}", n_res+m_res);
}
