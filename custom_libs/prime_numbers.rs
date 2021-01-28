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
        }
        while iter_num % i == 0 {
            res.push(i);
            iter_num /= i;
        }
    }
    res
}

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
