use std::io::stdin;
use std::str::FromStr;


// fn read<T: FromStr>() -> T {
//     let mut s = String::new();
//     // stdin $B4X?t(B -> Stdin $B9=B$BN(B -> readline $B4X?t(B
//     // .ok() $B$O(B Result enum $B$N4X?t$G!"(BReslt -> Option $B$K$9$k(B
//     // stdin().read_line(&mut s).ok();
//     stdin().read_line(&mut s).ok();
//     s.trim().parse().ok().unwrap()
// }
// 
// 
// fn main() {
//     let input: i32 = read();
//     println!("{}", input);
// }


fn read_vec<T: FromStr>() -> Vec<T> {
    let mut s = String::new();
    // stdin().read_line(&mut s).ok();
    stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}


fn main() {
    let input: Vec<i32> = read_vec();
    let dot = input[0] * input[1];

    if dot % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
