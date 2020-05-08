use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let mut x: i32 = s.trim().parse().unwrap();

    let mut res = 0;
    while x >= 5 {
        if x >= 500 {
            x -= 500;
            res += 1000;
        } else if x >= 5 {
            x -= 5;
            res += 5;
        }
    }

    println!("{}", res);
}
