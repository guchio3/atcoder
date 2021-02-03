use num::integer::gcd;
use proconio::input;

// fn get_prime_factors_eratosthenes(mut num: usize) -> Vec<usize> {
//     let mut smallest_map = vec![0; num + 1];
//     for i in 2..=num {
//         for j in 2..=num / i {
//             if smallest_map[i * j] == 0 {
//                 smallest_map[i * j] = i;
//             }
//         }
//     }
//     let mut res = vec![];
//     while smallest_map[num] != num {
//         res.push(num);
//         num /= smallest_map[num];
//     }
//     res.push(num);
//     res
// }

fn get_smallest_prime_map(num: usize) -> Vec<usize> {
    let mut smallest_map = vec![0; num + 1];
    for i in 2..=num {
        for j in 1..=num / i {
            if smallest_map[i * j] == 0 {
                smallest_map[i * j] = i;
            }
        }
    }
    smallest_map
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let a_max = a.iter().map(|x| *x).max().unwrap();
    let smallest_prime_map = get_smallest_prime_map(a_max);
    let mut used_map = vec![n; 1_000_001];
    let mut is_pairwise_coprime = true;
    'outer: for i in 0..n {
        let a_i = a[i];
        if a_i == 1 {
            continue;
        }
        let mut mut_a_i = a_i;
        loop {
            if mut_a_i == 1
                || (used_map[smallest_prime_map[mut_a_i]] != n
                    && used_map[smallest_prime_map[mut_a_i]] != i)
            {
                is_pairwise_coprime = false;
                break 'outer;
            } else {
                used_map[smallest_prime_map[mut_a_i]] = i;
            }
            if smallest_prime_map[mut_a_i] == mut_a_i {
                break;
            }
            mut_a_i /= smallest_prime_map[mut_a_i];
        }
    }
    if is_pairwise_coprime {
        println!("pairwise coprime");
    } else {
        let mut coprime = a[0];
        for i in 1..n {
            coprime = gcd(coprime, a[i]);
        }
        if coprime == 1 {
            println!("setwise coprime");
        } else {
            println!("not coprime");
        }
    }
}
