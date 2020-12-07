use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    if n % 2 == 1 {
        println!("0");
    } else {
        let mut res = 0;
        let mut base = 10;
        while base <= n {
            res += n / base;
            base *= 5;
        }
        println!("{}", res);
    }
}
