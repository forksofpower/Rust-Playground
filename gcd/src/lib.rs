/// Calculates the greatest common divisor (GCD) of two unsigned 64-bit integers `n` and `m`.
///
/// ### Arguments
/// * `n` - An unsigned 64-bit integer.
/// * `m` - An unsigned 64-bit integer.
///
/// ### Panics
/// This function will panic if either `n` or `m` is zero.
///
/// ### Returns
/// The GCD of `n` and `m`.
///
/// ### Examples
/// ```
/// use crate::gcd;
///
/// let result = gcd(14, 15);
/// assert_eq!(result, 1);
/// ```
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Use the `assert` macro to ensure that neither `n` nor `m` is zero
    assert_ne!(n, 0);
    assert_ne!(m, 0);

    // Use a loop to repeatedly apply the Euclidean algorithm
    // until the remainder of `m` divided by `n` is zero
    while m != 0 {
        // If `m` is less than `n`, swap their values
        // old swap method
        // if m < n {
        //     let t = m;
        //     m = n;
        //     n = t;
        // }
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        // Calculate the remainder of `m` divided by `n`
        m %= n;
    }
    // The GCD is the final value of `n`
    n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }

    #[test]
    #[should_panic]
    fn bad_input() {
        // let result = std::panic::catch_unwind(|| gcd(0, 0));
        gcd(0, 0);
    }
}
