use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    }

    let mut res = vec![];
    for &ab_i in ab.iter() {
        let mut best_i = 0;
        let mut best_mann = 9_223_372_036_854_775_807i64;
        for (i, &cd_i) in cd.iter().enumerate() {
            let mann = (ab_i.0 - cd_i.0).abs() + (ab_i.1 - cd_i.1).abs();
            if mann < best_mann {
                best_i = i;
                best_mann = mann;
            }
        }
        res.push(best_i + 1);
    }

    for res_i in res {
        println!("{}", res_i);
    }
}
