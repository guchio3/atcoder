/// 素因数分解する関数
/// 素因数が全て入った Vec を返す
///
/// ex. 12 -> [2, 2, 3]
fn get_prime_factors(num: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut iter_num = num;
    for i in 2..=num {
        if i > iter_num {
            break;
        } else if i * i > num {
            res.push(iter_num);
            break;
        }
        while iter_num % i == 0 {
            res.push(i);
            iter_num /= i;
        }
    }
    res
}

// num 以下の素数を返す
fn get_prime_numbers_lt(num: usize) -> Vec<usize> {
    let mut res = vec![];
    'outer: for i in 2..num {
        for j in 0..res.len() {
            if i % res[j] == 0 {
                continue 'outer;
            }
        }
        res.push(i);
    }
    res
}


fn get_divisors_eratosthenes(mut num: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut smallest_map = vec![0; num + 1];
    for i in 2..=num {
        if smallest_map[i] == 0 {
            res.push(i);
        }
        for j in 1..=num / i {
            if smallest_map[i * j] == 0 {
                smallest_map[i * j] = i;
            }
        }
    }
    res
    // let mut res = vec![];
    // while smallest_map[num] != num {
    //     res.push(smallest_map[num]);
    //     num /= smallest_map[num];
    // }
    // res.push(num);
    // res
}
