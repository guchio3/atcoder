use::proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        t: usize,
    }
    println!("{}", t * (n / x + ((n % x != 0) as usize)));
}
