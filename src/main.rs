mod reference;

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
    group.finish();
    criterion.final_summary();
}

#[cfg(test)]
mod test {
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
}
