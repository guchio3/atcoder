use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }
    let mut res = 0;
    let mut highest_height = 0;
    for &a_i in a.iter() {
        if a_i > highest_height {
            highest_height = a_i;
        }
        res += highest_height - a_i;
    }

    println!("{}", res);
}
