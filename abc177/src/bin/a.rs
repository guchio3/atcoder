use proconio::input;

fn main() {
    input!{
        d: usize,
        t: usize,
        s: usize,
    }
    if s * t >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
