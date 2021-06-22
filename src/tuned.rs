#[inline(never)] // Ensure this exist as a symbol
pub fn div1023(numerator: &mut [u64; 4]) -> u64 {
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
