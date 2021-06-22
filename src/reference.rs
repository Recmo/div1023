use core::convert::TryFrom;

const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

const fn mul_2(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}

/// Compute <hi, lo> / d, returning the quotient and the remainder.
// TODO: Make sure it uses divq on x86_64.
// See http://lists.llvm.org/pipermail/llvm-dev/2017-October/118323.html
// (Note that we require d > hi for this)
// TODO: If divq is not supported, use a fast software implementation:
// See https://gmplib.org/~tege/division-paper.pdf
pub fn divrem_2by1(lo: u64, hi: u64, d: u64) -> (u64, u64) {
    debug_assert!(d > 0);
    debug_assert!(d > hi);
    let d = u128::from(d);
    let n = val_2(lo, hi);
    let q = n / d;
    let r = n % d;
    debug_assert!(q < val_2(0, 1));
    debug_assert!(
        mul_2(u64::try_from(q).unwrap(), u64::try_from(d).unwrap())
            + val_2(u64::try_from(r).unwrap(), 0)
            == val_2(lo, hi)
    );
    debug_assert!(r < d);
    // There should not be any truncation.
    #[allow(clippy::cast_possible_truncation)]
    (q as u64, r as u64)
}

pub fn divrem_nby1(numerator: &mut [u64], divisor: u64) -> u64 {
    debug_assert!(divisor > 0);
    let mut remainder = 0;
    for i in (0..numerator.len()).rev() {
        let (ni, ri) = divrem_2by1(numerator[i], remainder, divisor);
        numerator[i] = ni;
        remainder = ri;
    }
    remainder
}

#[inline(never)] // Ensure this exist as a symbol
pub fn div1023(numerator: &mut [u64; 4]) -> u64 {
    divrem_nby1(numerator, 1023)
}
