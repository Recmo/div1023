//! See <https://gmplib.org/~tege/division-paper.pdf>

use core::intrinsics::unlikely;

const D: u64 = 1023;
const R: u64 = 0x0040100401004010; // Floor(2^64 / D)

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
    if unlikely(r > D) {
        dbg!();
        q += 1;
        r -= D;
    }
    assert!(r < D);
    (q, r)
}

fn divrem_2by1(lo: u64, hi: u64) -> (u64, u64) {
    let d = D << 54;
    let v = R;
    let u1 = (hi << 54) | (lo >> 10);
    let u0 = lo << 54;
    let u = val_2(u0, u1);
    let q = (u1 as u128) * (R as u128) + u;
    let (q0, q1) = (q as u64, (q >> 64) as u64);
    let mut q = q1.wrapping_add(1);
    let mut r = u0.wrapping_sub(q1.wrapping_mul(d));
    if r > q0 {
        q = q.wrapping_sub(1);
        r = r.wrapping_add(d);
    }
    if unlikely(r > d) {
        q += 1;
        r -= d;
    }
    r >>= 54;
    r += 1; // Pourquoi?
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
