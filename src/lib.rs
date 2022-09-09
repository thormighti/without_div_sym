//! # Without Divisor Symbol
//!
//!
//! This crate is just a fun play not to be taken serious. if you realy want to do division,
//! just use the `divisor(/)` symbol in your program. This is just for learning purposes.
//!

extern crate num;

pub use self::wo_divisor::divide;
pub mod wo_divisor {

    use num::{one, zero, One, Zero};

    /// contains the geneic function  which takes generic type of 2 numbers and returns their resut
    /// when divided
    ///
    /// # Example
    ///
    ///
    /// ```
    /// let a  = 7;
    /// let b = 2;
    ///
    /// assert_eq!(Some(3), without_div_sym::divide(a,b));
    /// ```
    ///    
    ///
    /// # Problems
    ///
    /// This crate doesnt accept floating point number yet. i will work on that on next versions.
    /// the example below will cause an error
    ///
    /// ```
    /// let a = 15.0;
    /// let b = 5.0;
    /// assert_eq!(some(3.0), without_div_sym::divide(a,b)); // This wont complie, it will bring out an error
    ///
    ///```
    ///

    pub fn divide<T>(mut dividend: T, mut divisor: T) -> Option<T>
    where
        T: std::cmp::PartialOrd
            + std::ops::Not<Output = T>
            + std::ops::Add<T, Output = T>
            + std::ops::ShlAssign<T>
            + std::ops::SubAssign<T>
            + One
            + Copy
            + Zero,
    {
        //when the divisor is zero

        let checkzero: T = zero();
        let theone: T = one();

        if divisor == checkzero {
            // None
            return None;
        }
        //to handle negative numbers
        let mut is_negative = false;
        if dividend < checkzero {
            //change the sign
            dividend = !dividend + theone;
            is_negative = !is_negative;
        }

        if divisor < checkzero {
            divisor = !divisor + theone;
            is_negative = !is_negative;
        }

        let mut result: T = one();
        let mut denominator = divisor;
        // Double divisor value with bitwise shift until bigger than dividend

        while dividend > denominator {
            denominator <<= theone;
            result <<= theone;
        }
        // Subtract divisor value until denominator is smaller than dividend
        while denominator > dividend {
            denominator -= divisor;
            result -= one();
        }

        // If one of dividend or divisor was negative, change sign of result
        if is_negative {
            result = !result + theone;
        }
        Some(result)
    }
}
