use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut i = 1;
    for j in 0..n {
        if a[j] == i {
            i += 1;
        }
    }
    if i == 1 {
        println!("-1");
    } else {
        println!("{}", n - (i - 1));
    }
}
