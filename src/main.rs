//! See <https://danlark.org/2020/06/14/128-bit-division/>

#![feature(asm)]

mod asm;
mod mgdiv;
mod reference;
mod tuned;

use criterion::{black_box, Criterion};
use rand::random;

fn main() {
    let n: [u64; 4] = random();
    let mut criterion = Criterion::default();
    let mut group = criterion.benchmark_group("div1023");
    group.bench_function("reference", move |bencher| {
        bencher.iter(|| {
            let n = black_box(n);
            let mut result = n.clone();
            let remainder = reference::div1023(&mut result);
            black_box(remainder)
        });
    });
    group.bench_function("tuned", move |bencher| {
        bencher.iter(|| {
            let n = black_box(n);
            let mut result = n.clone();
            let remainder = tuned::div1023(&mut result);
            black_box(remainder)
        });
    });
    group.bench_function("asm", move |bencher| {
        bencher.iter(|| {
            let n = black_box(n);
            let mut result = n.clone();
            let remainder = asm::div1023(&mut result);
            black_box(remainder)
        });
    });
    /*
    group.bench_function("mgdiv", move |bencher| {
        bencher.iter(|| {
            let n = black_box(n);
            let mut result = n.clone();
            let remainder = mgdiv::div1023(&mut result);
            black_box(remainder)
        });
    });
    */
    group.finish();
    criterion.final_summary();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reference() {
        let n = [
            0x30fa7e8b3b6ef8ed,
            0xfcbfcecabb6cc7fa,
            0x7d2cd0c79600b90c,
            0x01334da91c537060,
        ];
        let mut result = n.clone();
        let remainder = reference::div1023(&mut result);
        assert_eq!(remainder, 140);
        assert_eq!(
            result,
            [
                0xccff7e7f429f839f,
                0x789d574985101f39,
                0x5033580a34729cd5,
                0x00004ce6a3f010e0
            ]
        );
    }

    #[test]
    fn test_tuned() {
        for _ in 0..1000 {
            let n: [u64; 4] = random();
            let mut expected_result = n.clone();
            let expected_remainder = reference::div1023(&mut expected_result);
            let mut result = n.clone();
            let remainder = tuned::div1023(&mut result);
            assert_eq!(result, expected_result);
            assert_eq!(remainder, expected_remainder);
        }
    }

    #[test]
    fn test_asm() {
        for _ in 0..1000 {
            let n: [u64; 4] = random();
            let mut expected_result = n.clone();
            let expected_remainder = reference::div1023(&mut expected_result);
            let mut result = n.clone();
            let remainder = asm::div1023(&mut result);
            assert_eq!(result, expected_result);
            assert_eq!(remainder, expected_remainder);
        }
    }

    #[test]
    fn test_mgdiv() {
        for _ in 0..1000 {
            let n: [u64; 4] = random();
            let mut expected_result = n.clone();
            let expected_remainder = reference::div1023(&mut expected_result);
            let mut result = n.clone();
            let remainder = mgdiv::div1023(&mut result);
            assert_eq!(result, expected_result);
            assert_eq!(remainder, expected_remainder);
        }
    }
}
