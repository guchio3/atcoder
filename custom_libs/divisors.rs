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
