use num::traits::{NumAssign, PrimInt};
use std::fmt::Debug;

fn dec_to_oct_string<T>(decimal: T) -> String
where
    T: PrimInt + NumAssign + ToString,
{
    let mut octal_string: String = String::from("");
    let mut base = decimal;
    while base > T::zero() {
        let mut tmp_string = (base % T::from(8).unwrap()).to_string();
        tmp_string.push_str(octal_string.as_str());
        octal_string = tmp_string;

        base /= T::from(8).unwrap();
    }
    octal_string
}
