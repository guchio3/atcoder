use proconio::input;
use num::traits::{NumAssign, PrimInt};

fn get_divisors<T>(num: T) -> Vec<T>
where
    T: PrimInt + NumAssign,
{
    let mut i = T::one();
    let mut res = vec![];
    while i * i <= num {
        if num % i == T::zero() {
            let rev_i = num / i;
            res.push(i);
            if i != rev_i {
                res.push(rev_i);
            }
        }
        i += T::one();
    }
    res
}

fn main() {
    input! {
        n: usize
    }

    let mut divisors = get_divisors(n);
    divisors.sort();

    let mut res = 0;
    for divisor in divisors {
        let n_div = n / divisor;
        if divisor >= n_div {
            // println!("{}", divisor);
            res = (divisor as f64).log10() as usize + 1;
            break;
        }
    }
    println!("{}", res);
}
