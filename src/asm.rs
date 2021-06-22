// See <https://www.agner.org/optimize/instruction_tables.pdf>
// See <https://www.felixcloutier.com/x86/idiv>
// See `cargo asm div1023::asm::div1023`
// See `objdump ./target/release/div1023 --disassemble-symbols=___udivti3`

fn divrem_2by1(lo: u64, hi: u64) -> (u64, u64) {
    let d: u64 = 1023;
    let q: u64;
    let r: u64;
    unsafe {
        asm!(r"
            div {0}
        ",
        in(reg) d,
        inlateout("rax") lo => q,
        inlateout("rdx") hi => r,
        options(pure, nomem, nostack)
        );
    }
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
