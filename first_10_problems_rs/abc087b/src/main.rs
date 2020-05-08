use std::io::stdin;

// fn cnt_combinations(n: u32, k: u32) -> u32 {
//     let mut numerator = 1;
//     for i in n..n-k {numerator *= i;}
//     let mut denominator = 1;
//     for i in 1..k+1 {denominator *= i;}
//     // must be integer
//     numerator / denominator
// }
// 
// fn main() {
//     // $B"-(B 1 $B$K$J$k(B
//     // println!("{}", 10 / 9);
// 
//     const A_UNIT: u32 = 50;
//     const B_UNIT: u32 = 100;
//     const C_UNIT: u32 = 500;
//     
//     // parse inputs
//     let mut s = String::new();
// 
//     stdin().read_line(&mut s).ok();
//     let mut a: u32 = s.trim().parse().ok().unwrap();
//     s.clear();
// 
//     stdin().read_line(&mut s).ok();
//     let mut b: u32 = s.trim().parse().ok().unwrap();
//     s.clear();
// 
//     stdin().read_line(&mut s).ok();
//     let mut c: u32 = s.trim().parse().ok().unwrap();
//     s.clear();
// 
//     stdin().read_line(&mut s).ok();
//     let x: u32 = s.trim().parse().ok().unwrap();
// 
//     // a, b, c $B$rEy>uBV8r49(B
//     let a2b = a / (B_UNIT / A_UNIT);
//     a = a % (B_UNIT / A_UNIT);
// 
//     let b2c = b / (C_UNIT / B_UNIT);
//     let a2c = a2b / (C_UNIT / B_UNIT);
//     b = b % (C_UNIT / B_UNIT);
// 
//     // count patterns
//     // 1. 500 -> 100 -> 50 $B$G(B "$BH"(B" $B$r7h$a$k(B
//     // 2. "$BH"(B" $B$NMxMQ%Q%?!<%s$r7h$a$k(B
//     let cn = ;
// }

fn main() {
    const A_UNIT: u32 = 500;
    const B_UNIT: u32 = 100;
    const C_UNIT: u32 = 50;
    
    // parse inputs
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let a: u32 = s.trim().parse().ok().unwrap();
    s.clear();

    stdin().read_line(&mut s).ok();
    let b: u32 = s.trim().parse().ok().unwrap();
    s.clear();

    stdin().read_line(&mut s).ok();
    let c: u32 = s.trim().parse().ok().unwrap();
    s.clear();

    stdin().read_line(&mut s).ok();
    let x: u32 = s.trim().parse().ok().unwrap();

    let mut cnt = 0;

    // println!("A: {}, B: {}, C: {}, x: {}", a, b, c, x);

    for a_i in 0..a+1 {
        for b_i in 0..b+1 {
            for c_i in 0..c+1 {
                if (A_UNIT * a_i + B_UNIT * b_i + C_UNIT * c_i) == x {
                        // println!("{}, {}, {}", a_i, b_i, c_i);
                        // println!("{}", A_UNIT * a_i + B_UNIT * b_i + C_UNIT * c_i);
                        cnt += 1;
                    }
            }
        }
    }

    println!("{}", cnt);
}
