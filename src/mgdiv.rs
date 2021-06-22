/// See <https://gmplib.org/~tege/division-paper.pdf>

const D: u64 = 1023;
const R: u64 = 0x40100401004010; // Floor(2^64 / D)

const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

const fn umul(a: u64, b: u64) -> (u64, u64) {
    let product = (a as u128) * (b as u128);
    (product as u64, (product >> 64) as u64)
}

fn divrem_1by1(n: u64) -> (u64, u64) {
    let mut q = umul(n, R).1;
    let mut r = n - q * D;
    if r > D {
        q += 1;
        r -= D;
    }
    (q, r)
}

fn divrem_2by1(lo: u64, hi: u64) -> (u64, u64) {
    dbg!(lo, hi);
    assert!(hi < D);
    let n = (hi << 53) | (lo >> 10);
    dbg!(n);
    let q2 = (n as u128) * (R as u128);
    dbg!(q2);
    let q2 = (q2 >> 54) as u64;

    let n = val_2(lo, hi);
    let q = (n / D as u128) as u64;
    let r = (n % D as u128) as u64;

    assert_eq!(q, q2);

    (q, r)
}

#[inline(never)]
pub fn div1023(numerator: &mut [u64; 4]) -> u64 {
    let mut remainder = 0;
    let (ni, remainder) = divrem_1by1(numerator[3]);
    numerator[3] = ni;
    let (ni, remainder) = divrem_2by1(numerator[2], remainder);
    numerator[2] = ni;
    let (ni, remainder) = divrem_2by1(numerator[1], remainder);
    numerator[1] = ni;
    let (ni, remainder) = divrem_2by1(numerator[0], remainder);
    numerator[0] = ni;
    remainder
}
