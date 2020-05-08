use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let s: String = s.trim().parse().unwrap();

    if (s.chars().nth(2) == s.chars().nth(3)) & (s.chars().nth(4) == s.chars().nth(5)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
