use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().ok().unwrap();
    s.clear();

    let mut mochis: Vec<i32> = Vec::new();
    for _ in 0..n {
        stdin().read_line(&mut s).ok();
        let mochi: i32 = s.trim().parse().ok().unwrap();
        mochis.push(mochi);
        s.clear();
    }
    mochis.sort();

    let mut cnt = 0;
    let mut temp_mochi = -1;
    for mochi in mochis {
        if mochi != temp_mochi {
             cnt += 1;
        }
        temp_mochi = mochi;
    }

    println!("{}", cnt);
}
