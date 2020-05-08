use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();

    let mut cnt = 0;
    for i in 0..3{
        cnt += s.chars().nth(i).unwrap() as i32 - 48;
    }

    println!("{}", cnt)
}
