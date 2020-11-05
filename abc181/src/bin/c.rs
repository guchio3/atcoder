use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    }
    let mut used_lines = vec![];
    for i in 0..n {
        let x_i: f64;
        let y_i: f64;
        let xy_i = xy[i];
        x_i = xy_i.0;
        y_i = xy_i.1;
        for j in 0..i {
            let x_j: f64;
            let y_j: f64;
            let xy_j = xy[j];
            x_j = xy_j.0;
            y_j = xy_j.1;
            let line;
            if x_i - x_j == 0. {
                line = (None, Some(x_i));
            } else {
                let a = (y_i - y_j) / (x_i - x_j);
                let b = y_i - a * x_i;
                line = (Some(a), Some(b));
            }
            // println!("{:?}", (x_i, y_i));
            // println!("{:?}", line);
            if used_lines.contains(&line) {
                // println!("{:?}", (x_i, y_i));
                // println!("{:?}", line);
                // println!("{:?}", used_lines);
                println!("Yes");
                return;
            }
            used_lines.push(line);
        }
    }
    println!("No");
}
