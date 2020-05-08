use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let nums: Vec<i64> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    s.clear();
    let x = nums[0];
    let y = nums[1];

    stdin().read_line(&mut s).ok();
    let mut ps: Vec<i64> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    s.clear();
    ps.sort();
    ps.reverse();
    ps = ps[..(x as usize)].to_vec();

    stdin().read_line(&mut s).ok();
    let mut qs: Vec<i64> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    s.clear();
    qs.sort();
    qs.reverse();
    qs = qs[..(y as usize)].to_vec();

    stdin().read_line(&mut s).ok();
    let mut rs: Vec<i64> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    s.clear();
    rs.sort();

    ps.append(&mut qs);
    // ps.append(&mut rs);
    ps.sort();
    // ps.reverse();

    rs.reverse();
    let mut res_i: Option<usize> = None;
    for i in 0..rs.len() {
        if rs[i] > ps[i] {
            res_i = Some(i);
        } else {
            break;
        }
    }

    let res: i64;
    if let None = res_i {
        res = ps.iter().sum::<i64>();
    } else {
        res = (ps[res_i.unwrap() + 1..].iter().sum::<i64>()
            + rs[..res_i.unwrap() + 1].iter().sum::<i64>()) as i64;
    }

    // 後は、要は bottom rs.len() 個の ps について
    // allignment の問題
    // let mut switch_cnt = 0;
    // let mut rs_i = 0;
    // for ps_item in ps.iter_mut() {
    //     while rs_i < rs.len() {
    //         if ps_item < &mut rs[rs_i] {
    //             // *ps_item = rs[rs_i];
    //             switch_cnt += 1;
    //             rs_i += 1;
    //             break;
    //         }
    //         rs_i += 1;
    //     }
    // }
    // rs.reverse();
    // 
    // let res = ps[switch_cnt..].iter().sum::<i64>() + rs[..switch_cnt].iter().sum::<i64>();

    //     for res_elem in ps.iter_mut() {
    //         for rs_i in 0..rs.len() {
    //             if rs[i] {
    //                 rs = rs[rs_i..].to_vec();
    //                 break;
    //             }
    //         }
    //     }
    //
    //
    //
    //         if rs.len() > 0 {
    //             if *res_item < *rs.peek().unwrap() {
    //                 *res_item = rs.pop().unwrap();
    //             } else {
    //                 break;
    //             }
    //         } else {
    //             break;
    //         }
    //     }

    // let res: i64 = ps[..(x + y) as usize].to_vec().iter().sum();

    println!("{}", res);
}
