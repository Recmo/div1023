use core::intrinsics::unlikely;

const D: u64 = 1023;
const DN: u64 = 1023 << 54;
const R: u64 = 0x0040100401004010; // Floor(2^64 / D)

const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

const fn umul(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}

fn divrem_1by1(n: u64) -> (u64, u64) {
    let mut q = (umul(n, R) >> 64) as u64;
    let mut r = n - q * D;
    if unlikely(r > D) {
        dbg!();
        q += 1;
        r -= D;
    }
    assert!(r < D);
    (q, r)
}

/// Algorithm 1 in the paper.
fn divrem_2by1_mg(u0: u64, u1: u64, d: u64, v: u64) -> (u64, u64) {
    debug_assert!(d >= 1_u64 << 63);
    debug_assert!(u1 < d);
    let mut q = ((umul(v, u1) >> 64) as u64) + u1;
    let p = umul(q, d);
    let mut r = val_2(u0, u1) - p;
    if r >= (d as u128) {
        q += 1;
        r -= d as u128;
    }
    if r >= (d as u128) {
        q += 1;
        r -= d as u128;
    }
    if r >= (d as u128) {
        q += 1;
        r -= d as u128;
    }
    debug_assert!(r >> 64 == 0);
    let r = r as u64;
    debug_assert!(r < d);
    debug_assert_eq!(val_2(u0, u1), umul(q, d) + (r as u128));
    (q, r)
}

/// Algorithm 4 in the paper.
/// TODO: Fix
fn divrem_2by1_mg_4(u0: u64, u1: u64, d: u64, v: u64) -> (u64, u64) {
    debug_assert!(d >= 1_u64 << 63);
    debug_assert!(u1 < d);

    dbg!(u0, u1, d, v);
    let q = umul(v, u1) + val_2(u0, u1);
    dbg!(q);

    let (q0, q1) = (q as u64, (q >> 64) as u64);
    dbg!(q0, q1);

    let mut q = q1.wrapping_add(1);
    let mut r = u0.wrapping_sub(q1.wrapping_mul(d));
    dbg!(q, r);

    if r > q0 {
        q = q.wrapping_sub(1);
        r = r.wrapping_add(d);
        dbg!(q, r);
    }

    if unlikely(r >= d) {
        q += 1;
        r -= d;
        dbg!(q, r);
    }
    dbg!(q, r);

    assert!(r < d);
    // assert_eq!(val_2(u0, u1), umul(q, d) + (r as u128));
    (q, r)
}

fn divrem_2by1(lo: u64, hi: u64) -> (u64, u64) {
    let d = D << 54;
    let v = R;
    let u1 = (hi << 54) | (lo >> 10);
    let u0 = lo << 54;

    let (q2, r2) = divrem_2by1_mg(u0, u1, d, v);
    let (q, mut r) = divrem_2by1_mg(u0, u1, d, v);
    assert_eq!(q, q2);
    assert_eq!(r, r2);

    r >>= 54;
    (q, r)
}

#[inline(never)] // Ensure this exist as a symbol
pub fn div1023(numerator: &mut [u64; 4]) -> u64 {
    let remainder = 0;
    let (ni, remainder) = divrem_2by1(numerator[3], remainder);
    numerator[3] = ni;
    let (ni, remainder) = divrem_2by1(numerator[2], remainder);
    numerator[2] = ni;
    let (ni, remainder) = divrem_2by1(numerator[1], remainder);
    numerator[1] = ni;
    let (ni, remainder) = divrem_2by1(numerator[0], remainder);
    numerator[0] = ni;
    remainder
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reference;
    use proptest::proptest;

    #[test]
    fn test_reciprocal() {
        // Use the formula in III.A
        let n = val_2(u64::MAX, u64::MAX - DN);
        let v_computed = n / (DN as u128);
        assert_eq!(R as u128, v_computed)
    }

    #[test]
    fn test_div_2by1_mg_zero() {
        let u0 = 0;
        let u1 = 0;
        let expected = reference::divrem_2by1(u0, u1, DN);
        let result = divrem_2by1_mg(u0, u1, DN, R);
        dbg!(result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_div_2by1_mg() {
        proptest!(|(u0 in 0..=u64::MAX, u1 in 0_u64..DN)| {
            let expected = reference::divrem_2by1(u0, u1, DN);
            let result = divrem_2by1_mg(u0, u1, DN, R);
            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_div_2by1() {
        proptest!(|(lo in 0..=u64::MAX, hi in 0_u64..D)| {
            let expected = reference::divrem_2by1(lo, hi, D);
            let result = divrem_2by1(lo, hi);
            assert_eq!(result, expected);
        });
    }
}
