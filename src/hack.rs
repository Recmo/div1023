/// Compute a += b + carry, returning the new carry over.
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let sum = (*a as u128) + (b as u128) + (carry as u128);
    *a = sum as u64;
    (sum >> 64) as u64
}

fn shift_right(numerator: &mut [u64; 4], n: usize) {
    debug_assert!(n > 0);
    debug_assert!(n < 64);
    let m = 64 - n;
    numerator[0] >>= n;
    numerator[0] |= numerator[1] << m;
    numerator[1] >>= n;
    numerator[1] |= numerator[2] << m;
    numerator[2] >>= n;
    numerator[2] |= numerator[3] << m;
    numerator[3] >>= n;
}

fn add_shift_right(n: &mut [u64; 4], s: usize) {
    if s < 64 {
        let t = 64 - s;
        let shift = (n[0] >> s) | (n[1] << t);
        let carry = adc(&mut n[0], shift, 0);
        let shift = (n[1] >> s) | (n[2] << t);
        let carry = adc(&mut n[1], shift, carry);
        let shift = (n[2] >> s) | (n[3] << t);
        let carry = adc(&mut n[2], shift, carry);
        let shift = n[3] >> s;
        let _ = adc(&mut n[3], shift, carry);
    } else if s < 128 {
        let s = s - 64;
        let t = 64 - s;
        let shift = (n[1] >> s) | (n[2] << t);
        let carry = adc(&mut n[0], shift, 0);
        let shift = (n[2] >> s) | (n[3] << t);
        let carry = adc(&mut n[1], shift, carry);
        let shift = n[3] >> s;
        let carry = adc(&mut n[2], shift, carry);
        let _ = adc(&mut n[3], 0, carry);
    } else if s < 192 {
        let s = s - 128;
        let t = 64 - s;
        let shift = (n[2] >> s) | (n[3] << t);
        let carry = adc(&mut n[0], shift, 0);
        let shift = n[3] >> s;
        let carry = adc(&mut n[1], shift, carry);
        let carry = adc(&mut n[2], 0, carry);
        let _ = adc(&mut n[3], 0, carry);
    }
}

pub fn quotient(numerator: &mut [u64; 4]) {
    // Multiply by 0x0.(00401) repeated
    shift_right(numerator, 10);
    add_shift_right(numerator, 10);
    add_shift_right(numerator, 20);
    add_shift_right(numerator, 40);
    add_shift_right(numerator, 80);
    add_shift_right(numerator, 160);
    // eprintln!(
    //     "{:0>16x}{:0>16x}{:0>16x}{:0>16x}",
    //     numerator[3], numerator[2], numerator[1], numerator[0]
    // );
}

#[inline(never)] // Ensure this exist as a symbol
pub fn div1023(numerator: &mut [u64; 4]) -> u64 {
    let remainder = numerator[0];
    quotient(numerator);
    remainder.wrapping_sub(numerator[0].wrapping_mul(1023))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reference;
    use proptest::{prelude::any, proptest};

    #[test]
    fn test_hack_one() {
        let n = [u64::MAX; 4];
        let mut expected_result = n.clone();
        let expected_remainder = reference::div1023(&mut expected_result);
        let mut result = n.clone();
        let remainder = div1023(&mut result);
        assert_eq!(result, expected_result);
        assert_eq!(remainder, expected_remainder);
    }

    #[test]
    fn test_hack() {
        proptest!(|(n in any::<[u64;4]>())| {
            let mut expected_result = n.clone();
            let expected_remainder = reference::div1023(&mut expected_result);
            let mut result = n.clone();
            let remainder = div1023(&mut result);
            assert_eq!(result, expected_result);
            assert_eq!(remainder, expected_remainder);
        });
    }
}
