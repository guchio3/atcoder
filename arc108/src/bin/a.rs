use proconio::input;

fn main() {
    input! {
        s: i64, p: i64
    }
    for n in 1..=1_000_000 {
        if ((p % n) == 0) && (s > n) && (p % (s - n) == 0) && (n * (s - n) == p) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
