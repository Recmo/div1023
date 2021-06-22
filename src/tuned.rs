const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

fn divrem_2by1(lo: u64, hi: u64) -> (u64, u64) {
    let D: u128 = 1023;
    let n = val_2(lo, hi);
    let q = (n / D) as u64;
    let r = (n % D) as u64;
    (q, r)
}

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

// Fully inlined version has no advantage, compiler does the same
fn _div1023_inlined(numerator: &mut [u64; 4]) -> u64 {
    const D: u128 = 1023;

    let mut n = numerator[3] as u128;
    numerator[3] = (n / D) as u64;
    n %= D;
    n <<= 64;
    n |= numerator[2] as u128;
    numerator[2] = (n / D) as u64;
    n %= D;
    n <<= 64;
    n |= numerator[1] as u128;
    numerator[1] = (n / D) as u64;
    n %= D;
    n <<= 64;
    n |= numerator[0] as u128;
    numerator[0] = (n / D) as u64;
    (n % D) as u64
}
