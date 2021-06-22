# Dividide by 1023

## Benchmark

```
cargo run --release
```

```
div1023/reference       time:   [77.923 ns 78.617 ns 79.366 ns]
div1023/tuned           time:   [77.140 ns 77.758 ns 78.454 ns]
div1023/asm             time:   [75.786 ns 76.361 ns 76.992 ns]
div1023/mgdiv           time:   [33.080 ns 33.304 ns 33.542 ns]
```

## Generated assembly

<details>
<summary><code>cargo asm div1023::asm::div1023</code></summary>

```asm
 push    rbp
 mov     rbp, rsp
 mov     rax, qword, ptr, [rdi, +, 24]
 mov     esi, 1023
 xor     edx, edx
 div     rsi
 mov     qword, ptr, [rdi, +, 24], rax
 mov     rax, qword, ptr, [rdi, +, 16]
 div     rsi
 mov     qword, ptr, [rdi, +, 16], rax
 mov     rax, qword, ptr, [rdi, +, 8]
 div     rsi
 mov     rcx, rax
 mov     rax, qword, ptr, [rdi]
 div     rsi
 mov     qword, ptr, [rdi, +, 8], rcx
 mov     qword, ptr, [rdi], rax
 mov     rax, rdx
 pop     rbp
 ret
```

</details>

<details>
<summary><code>cargo asm div1023::mgdiv::div1023</code></summary>

```asm
 push    rbp
 mov     rbp, rsp
 push    r15
 push    r14
 push    r13
 push    r12
 push    rbx
 mov     rsi, qword, ptr, [rdi, +, 24]
 mov     rcx, rsi
 shr     rcx, 10
 shl     rsi, 54
 movabs  r15, 18032007892189200
 mov     rax, rcx
 mul     r15
 mov     r8, rdx
 add     r8, rcx
 movabs  r11, 18014398509481984
 mov     rax, r8
 mul     r11
 sub     rdx, r8
 add     rax, rsi
 adc     rdx, rcx
 mov     rcx, rax
 add     rcx, r11
 mov     rsi, rdx
 adc     rsi, -1
 xor     r14d, r14d
 movabs  r12, -18014398509481985
 cmp     r12, rax
 mov     ebx, 0
 sbb     rbx, rdx
 setb    bl
 adc     r8, 0
 test    bl, bl
 cmove   rsi, rdx
 test    bl, bl
 cmove   rcx, rax
 mov     rax, rcx
 add     rax, r11
 mov     rdx, rsi
 adc     rdx, -1
 cmp     r12, rcx
 mov     ebx, 0
 sbb     rbx, rsi
 setb    bl
 adc     r8, 0
 test    bl, bl
 cmove   rdx, rsi
 test    bl, bl
 cmove   rax, rcx
 lea     rcx, [rax, +, r11]
 cmp     r12, rax
 mov     esi, 0
 sbb     rsi, rdx
 setb    dl
 adc     r8, 0
 test    dl, dl
 cmove   rcx, rax
 shr     rcx, 54
 mov     rsi, qword, ptr, [rdi, +, 16]
 shld    rcx, rsi, 54
 mov     rax, rcx
 mul     r15
 mov     r9, rdx
 shl     rsi, 54
 add     r9, rcx
 mov     rax, r9
 mul     r11
 sub     rdx, r9
 add     rax, rsi
 adc     rdx, rcx
 mov     rcx, rax
 add     rcx, r11
 mov     rsi, rdx
 adc     rsi, -1
 cmp     r12, rax
 mov     ebx, 0
 sbb     rbx, rdx
 setb    bl
 adc     r9, 0
 test    bl, bl
 cmove   rsi, rdx
 test    bl, bl
 cmove   rcx, rax
 mov     rax, rcx
 add     rax, r11
 mov     rdx, rsi
 adc     rdx, -1
 cmp     r12, rcx
 mov     ebx, 0
 sbb     rbx, rsi
 setb    bl
 adc     r9, 0
 test    bl, bl
 cmove   rdx, rsi
 test    bl, bl
 cmove   rax, rcx
 lea     rsi, [rax, +, r11]
 cmp     r12, rax
 mov     ecx, 0
 sbb     rcx, rdx
 setb    cl
 adc     r9, 0
 test    cl, cl
 cmove   rsi, rax
 shr     rsi, 54
 mov     rcx, qword, ptr, [rdi, +, 8]
 shld    rsi, rcx, 54
 mov     rax, rsi
 mul     r15
 mov     r10, rdx
 shl     rcx, 54
 add     r10, rsi
 mov     rax, r10
 mul     r11
 sub     rdx, r10
 add     rax, rcx
 adc     rdx, rsi
 mov     rcx, rax
 add     rcx, r11
 mov     rsi, rdx
 adc     rsi, -1
 cmp     r12, rax
 mov     ebx, 0
 sbb     rbx, rdx
 setb    bl
 adc     r10, 0
 test    bl, bl
 cmove   rsi, rdx
 test    bl, bl
 cmove   rcx, rax
 mov     rax, rcx
 add     rax, r11
 mov     rdx, rsi
 adc     rdx, -1
 cmp     r12, rcx
 mov     ebx, 0
 sbb     rbx, rsi
 setb    bl
 adc     r10, 0
 test    bl, bl
 cmove   rdx, rsi
 mov     r13, qword, ptr, [rdi]
 test    bl, bl
 cmove   rax, rcx
 cmp     r12, rax
 mov     ecx, 0
 sbb     rcx, rdx
 lea     rcx, [rax, +, r11]
 setb    dl
 adc     r10, 0
 test    dl, dl
 cmove   rcx, rax
 shr     rcx, 54
 shld    rcx, r13, 54
 mov     rax, rcx
 mul     r15
 mov     r15, rdx
 shl     r13, 54
 add     r15, rcx
 mov     rax, r15
 mul     r11
 sub     rdx, r15
 add     rax, r13
 adc     rdx, rcx
 mov     rcx, rax
 add     rcx, r11
 mov     rbx, rdx
 adc     rbx, -1
 cmp     r12, rax
 mov     esi, 0
 sbb     rsi, rdx
 setb    sil
 adc     r15, 0
 test    sil, sil
 cmove   rbx, rdx
 test    sil, sil
 cmove   rcx, rax
 mov     rax, rcx
 add     rax, r11
 mov     rdx, rbx
 adc     rdx, -1
 cmp     r12, rcx
 mov     esi, 0
 sbb     rsi, rbx
 setb    sil
 adc     r15, 0
 test    sil, sil
 cmove   rdx, rbx
 test    sil, sil
 cmove   rax, rcx
 add     r11, rax
 cmp     r12, rax
 sbb     r14, rdx
 mov     qword, ptr, [rdi, +, 24], r8
 mov     qword, ptr, [rdi, +, 16], r9
 mov     qword, ptr, [rdi, +, 8], r10
 setb    cl
 adc     r15, 0
 mov     qword, ptr, [rdi], r15
 test    cl, cl
 cmovne  rax, r11
 shr     rax, 54
 pop     rbx
 pop     r12
 pop     r13
 pop     r14
 pop     r15
 pop     rbp
 ret
```

</details>

Turns out `reference` and `tuned` have identical assembly.

<details>
<summary><code>cargo asm div1023::reference::div1023</code></summary>

```asm
 push    rbp
 mov     rbp, rsp
 push    r15
 push    r14
 push    r12
 push    rbx
 mov     r15, rdi
 mov     rcx, qword, ptr, [rdi, +, 24]
 movabs  rdx, 18032007892189201
 mov     rax, rcx
 mul     rdx
 mov     rax, rcx
 sub     rax, rdx
 shr     rax
 add     rax, rdx
 shr     rax, 9
 mov     rsi, rax
 mov     qword, ptr, [rdi, +, 24], rax
 shl     rax, 10
 sub     rsi, rax
 add     rsi, rcx
 mov     rbx, qword, ptr, [rdi, +, 16]
 mov     edx, 1023
 mov     rdi, rbx
 xor     ecx, ecx
 call    ___udivti3
 add     ebx, eax
 mov     qword, ptr, [r15, +, 16], rax
 mov     r14, qword, ptr, [r15]
 mov     r12, qword, ptr, [r15, +, 8]
 and     ebx, 1023
 mov     edx, 1023
 mov     rdi, r12
 mov     rsi, rbx
 xor     ecx, ecx
 call    ___udivti3
 add     r12d, eax
 mov     qword, ptr, [r15, +, 8], rax
 and     r12d, 1023
 mov     edx, 1023
 mov     rdi, r14
 mov     rsi, r12
 xor     ecx, ecx
 call    ___udivti3
 mov     rcx, rax
 mov     qword, ptr, [r15], rax
 shl     rax, 10
 sub     rcx, rax
 add     rcx, r14
 mov     rax, rcx
 pop     rbx
 pop     r12
 pop     r14
 pop     r15
 pop     rbp
 ret
```
