const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

fn divrem_2by1(lo: u64, hi: u64) -> (u64, u64) {
    let d = 1023_u128;
    let n = val_2(lo, hi);
    let q = n / d;
    let r = n % d;
    (q as u64, r as u64)
}

// #[inline(never)]
pub fn div1023(numerator: &mut [u64; 4]) -> u64 {
    let mut remainder = 0;

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
