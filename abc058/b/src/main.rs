use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let o: Vec<char> = s.trim().chars().collect();
    s.clear();

    stdin().read_line(&mut s).ok();
    let e: Vec<char> = s.trim().chars().collect();

    let mut res = vec!['_'; o.len() + e.len()];
    if o.len() > e.len() {
        res[o.len() + e.len() - 1] = o[o.len() - 1];
    }

    for ((i, &o_i), e_i) in o.iter().enumerate().zip(e) {
        res[i * 2] = o_i;
        res[i * 2 + 1] = e_i;
    }

    let res_str: String = res.into_iter().collect();
    println!("{}", res_str);
}
