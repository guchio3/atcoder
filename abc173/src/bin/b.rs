use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;

    for s_i in s {
        if s_i == "AC" { ac += 1; }
        if s_i == "WA" { wa += 1; }
        if s_i == "TLE" { tle += 1; }
        if s_i == "RE" { re += 1; }
    }

    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}
