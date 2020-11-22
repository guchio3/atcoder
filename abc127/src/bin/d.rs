use proconio::input;

// fn main() {
//     input! {
//         n: usize, m: usize,
//         mut a: [i64; n],
//         mut bc: [(usize, i64); m]
//     }
// 
//     for a_i in a {
//         bc.push((1, a_i));
//     }
// 
//     bc.sort_by_key(|bc_i| -bc_i.1);
// 
//     let mut i = 0;
//     let mut res = 0;
//     'outer: for bc_i in bc {
//         for _ in 0..bc_i.0 {
//             if i > n - 1 {
//                 break 'outer;
//             }
//             res += bc_i.1;
//             i += 1;
//         }
//     }
// 
//     println!("{}", res);
// }

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [i64; n],
        mut bc: [(usize, i64); m]
    }

    // a.sort_by_key(|a_i| -a_i);
    a.sort();
    bc.sort_by_key(|pair| -pair.1);

    let mut i = 0;
    'outer: for bc_i in bc {
        for _ in 0..bc_i.0 {
            // println!("{}", bc_i.1);
            loop {
                if i > n - 1 {
                    break 'outer;
                }
                if a[i] < bc_i.1 {
                    a[i] = bc_i.1;
                    // println!("{:?}", a);
                    i += 1;
                    break;
                }
                i += 1;
            }
        }
    }

    let res: i64 = a.iter().sum();
    println!("{}", res);
}
