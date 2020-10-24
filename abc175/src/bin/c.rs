use proconio::input;

fn main() {
    input!{
        x: i64,
        k: i64,
        d: i64,
    }

    let res;

    if x.abs() / d > k {
        res = x.abs() - k * d;
    } else {
        if (k - (x.abs() / d)) % 2 == 0 {
            res = x.abs() % d;
        } else {
            res = d - x.abs() % d;
        }
    }

    println!("{}", res);
}
