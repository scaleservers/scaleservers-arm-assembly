// Copyright (c) Scaleservers LLC
//
// Generates a C harness that executes this encoder's bytes on real AArch32-capable hardware and checks the
// results against Rust-computed expectations. Each case
// is `<instruction(s)>; bx lr`, called as a `uint32_t f(uint32_t a, uint32_t b)` (AAPCS: a->r0, b->r1,
// result<-r0). BOTH instruction sets are swept: T32 (Thumb -- the function pointer carries the Thumb bit) and
// A32 (ARM state -- Thumb bit clear, so the call interworks to ARM state). This is the ARM32 analog of the
// ARM64 `hardware_exec_gen.rs`; together they give the encoder an end-to-end silicon oracle beyond the
// GNU/LLVM byte-match. Run: `cargo run --example arm32_hardware_exec_gen` -> writes `arm32_hardware_exec.c`.

use scaleservers_arm32_assembly::Arm32Condition::AlwaysUnconditional as AL;
use scaleservers_arm32_assembly::Arm32GeneralPurposeRegister as R;
use scaleservers_arm32_assembly::Arm32LowGeneralPurposeRegister as L;
use scaleservers_arm32_assembly::Arm32RegisterShift as Shift;
use scaleservers_arm32_assembly::ArmA32Instruction;
use scaleservers_arm32_assembly::ArmA32Instruction::*;
use scaleservers_arm32_assembly::ArmT32Instruction;
use scaleservers_arm32_assembly::ArmT32Instruction::*;

// Encode `instrs` then a Thumb `bx lr` -> flat blob (entered with the Thumb bit set).
fn t32(instrs: &[ArmT32Instruction]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for instruction in instrs {
        bytes.extend(instruction.encode().expect("t32 encode"));
    }
    bytes.extend(ArmT32Instruction::Bx_T1(R::R14).encode().expect("bx t1"));
    bytes
}

// Encode `instrs` then an ARM-state `bx lr` -> flat blob (entered with the Thumb bit clear -> interworks to A32).
fn a32(instrs: &[ArmA32Instruction]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for instruction in instrs {
        bytes.extend(instruction.encode().expect("a32 encode"));
    }
    bytes.extend(
        ArmA32Instruction::Bx_A1(AL, R::R14)
            .encode()
            .expect("bx a1"),
    );
    bytes
}

struct Case {
    name: &'static str,
    thumb: bool,
    bytes: Vec<u8>,
    a: u32,
    b: u32,
    expected: u32,
}

fn main() {
    // a few reusable operands for the bitwise / extend / reverse cases
    let pat = 0x0123_4567u32; // a nice byte-distinct value for rev / shift / extend
    let m1 = 0x0F0F_0F0Fu32;
    let m2 = 0x00FF_00FFu32;

    let cases = vec![
        // ============================ T32 (Thumb) ============================
        Case {
            name: "T32 adds r0,r0,r1",
            thumb: true,
            bytes: t32(&[Add_Register_T1(L::R0, L::R0, L::R1)]),
            a: 100,
            b: 200,
            expected: 300,
        },
        Case {
            name: "T32 subs r0,r0,r1",
            thumb: true,
            bytes: t32(&[Sub_Register_T1(L::R0, L::R0, L::R1)]),
            a: 500,
            b: 200,
            expected: 300,
        },
        Case {
            name: "T32 muls r0,r1",
            thumb: true,
            bytes: t32(&[Mul_T1(L::R0, L::R1)]),
            a: 6,
            b: 7,
            expected: 42,
        },
        Case {
            name: "T32 ands r0,r1",
            thumb: true,
            bytes: t32(&[And_Register_T1(L::R0, L::R1)]),
            a: m1,
            b: m2,
            expected: m1 & m2,
        },
        Case {
            name: "T32 orrs r0,r1",
            thumb: true,
            bytes: t32(&[Orr_Register_T1(L::R0, L::R1)]),
            a: m1,
            b: m2,
            expected: m1 | m2,
        },
        Case {
            name: "T32 eors r0,r1",
            thumb: true,
            bytes: t32(&[Eor_Register_T1(L::R0, L::R1)]),
            a: m1,
            b: m2,
            expected: m1 ^ m2,
        },
        Case {
            name: "T32 bics r0,r1",
            thumb: true,
            bytes: t32(&[Bic_Register_T1(L::R0, L::R1)]),
            a: m1,
            b: m2,
            expected: m1 & !m2,
        },
        Case {
            name: "T32 lsls r0,r1",
            thumb: true,
            bytes: t32(&[Lsl_Register_T1(L::R0, L::R1)]),
            a: pat,
            b: 4,
            expected: pat.wrapping_shl(4),
        },
        Case {
            name: "T32 lsrs r0,r1",
            thumb: true,
            bytes: t32(&[Lsr_Register_T1(L::R0, L::R1)]),
            a: 0xF000_0000,
            b: 4,
            expected: 0x0F00_0000,
        },
        Case {
            name: "T32 asrs r0,r1",
            thumb: true,
            bytes: t32(&[Asr_Register_T1(L::R0, L::R1)]),
            a: 0x8000_0000,
            b: 4,
            expected: ((0x8000_0000u32 as i32) >> 4) as u32,
        },
        Case {
            name: "T32 rors r0,r1",
            thumb: true,
            bytes: t32(&[Ror_Register_T1(L::R0, L::R1)]),
            a: 0x0000_00FF,
            b: 4,
            expected: 0x0000_00FFu32.rotate_right(4),
        },
        Case {
            name: "T32 mvns r0,r1",
            thumb: true,
            bytes: t32(&[Mvn_Register_T1(L::R0, L::R1)]),
            a: 0,
            b: 0x0000_FFFF,
            expected: !0x0000_FFFFu32,
        },
        Case {
            name: "T32 rev r0,r1",
            thumb: true,
            bytes: t32(&[Rev_T1(L::R0, L::R1)]),
            a: 0,
            b: pat,
            expected: pat.swap_bytes(),
        },
        Case {
            name: "T32 rev16 r0,r1",
            thumb: true,
            bytes: t32(&[Rev16_T1(L::R0, L::R1)]),
            a: 0,
            b: pat,
            expected: ((pat >> 8) & 0x00FF_00FF) | ((pat << 8) & 0xFF00_FF00),
        },
        Case {
            name: "T32 uxtb r0,r1",
            thumb: true,
            bytes: t32(&[Uxtb_T1(L::R0, L::R1)]),
            a: 0,
            b: 0x0123_45EF,
            expected: 0x0000_00EF,
        },
        Case {
            name: "T32 uxth r0,r1",
            thumb: true,
            bytes: t32(&[Uxth_T1(L::R0, L::R1)]),
            a: 0,
            b: 0x0123_45EF,
            expected: 0x0000_45EF,
        },
        Case {
            name: "T32 sxtb r0,r1",
            thumb: true,
            bytes: t32(&[Sxtb_T1(L::R0, L::R1)]),
            a: 0,
            b: 0x0000_0080,
            expected: ((0x80u8 as i8) as i32) as u32,
        },
        Case {
            name: "T32 sxth r0,r1",
            thumb: true,
            bytes: t32(&[Sxth_T1(L::R0, L::R1)]),
            a: 0,
            b: 0x0000_8000,
            expected: ((0x8000u16 as i16) as i32) as u32,
        },
        // ============================ A32 (ARM state) ============================
        Case {
            name: "A32 add r0,r0,r1",
            thumb: false,
            bytes: a32(&[Add_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::none(),
            )]),
            a: 100,
            b: 200,
            expected: 300,
        },
        Case {
            name: "A32 sub r0,r0,r1",
            thumb: false,
            bytes: a32(&[Sub_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::none(),
            )]),
            a: 500,
            b: 200,
            expected: 300,
        },
        Case {
            name: "A32 mul r0,r0,r1",
            thumb: false,
            bytes: a32(&[Mul_A1(AL, false, R::R0, R::R0, R::R1)]),
            a: 6,
            b: 7,
            expected: 42,
        },
        Case {
            name: "A32 mla r0,r0,r1,r0 (a*b+a)",
            thumb: false,
            bytes: a32(&[Mla_A1(AL, false, R::R0, R::R0, R::R1, R::R0)]),
            a: 6,
            b: 7,
            expected: 6u32.wrapping_mul(7).wrapping_add(6),
        },
        Case {
            name: "A32 and r0,r0,r1",
            thumb: false,
            bytes: a32(&[And_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::none(),
            )]),
            a: m1,
            b: m2,
            expected: m1 & m2,
        },
        Case {
            name: "A32 orr r0,r0,r1",
            thumb: false,
            bytes: a32(&[Orr_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::none(),
            )]),
            a: m1,
            b: m2,
            expected: m1 | m2,
        },
        Case {
            name: "A32 eor r0,r0,r1",
            thumb: false,
            bytes: a32(&[Eor_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::none(),
            )]),
            a: m1,
            b: m2,
            expected: m1 ^ m2,
        },
        Case {
            name: "A32 bic r0,r0,r1",
            thumb: false,
            bytes: a32(&[Bic_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::none(),
            )]),
            a: m1,
            b: m2,
            expected: m1 & !m2,
        },
        Case {
            name: "A32 mvn r0,r1",
            thumb: false,
            bytes: a32(&[Mvn_Register_A1(AL, false, R::R0, R::R1, Shift::none())]),
            a: 0,
            b: 0x0000_FFFF,
            expected: !0x0000_FFFFu32,
        },
        // the A32 barrel-shifter operand (immediate shift on Rm) -- a shape T32 narrow forms don't have
        Case {
            name: "A32 mov r0,r1,lsl #4",
            thumb: false,
            bytes: a32(&[Mov_Register_A1(AL, false, R::R0, R::R1, Shift::Lsl(4))]),
            a: 0,
            b: pat,
            expected: pat.wrapping_shl(4),
        },
        Case {
            name: "A32 mov r0,r1,asr #4",
            thumb: false,
            bytes: a32(&[Mov_Register_A1(AL, false, R::R0, R::R1, Shift::Asr(4))]),
            a: 0,
            b: 0x8000_0000,
            expected: ((0x8000_0000u32 as i32) >> 4) as u32,
        },
        Case {
            name: "A32 mov r0,r1,ror #8",
            thumb: false,
            bytes: a32(&[Mov_Register_A1(AL, false, R::R0, R::R1, Shift::Ror(8))]),
            a: 0,
            b: 0x0000_00FF,
            expected: 0x0000_00FFu32.rotate_right(8),
        },
        Case {
            name: "A32 add r0,r0,r1,lsl #2 (shifted)",
            thumb: false,
            bytes: a32(&[Add_Register_A1(
                AL,
                false,
                R::R0,
                R::R0,
                R::R1,
                Shift::Lsl(2),
            )]),
            a: 0x1000,
            b: 0x10,
            expected: 0x1000u32.wrapping_add(0x10u32.wrapping_shl(2)),
        },
        Case {
            name: "A32 rev r0,r1",
            thumb: false,
            bytes: a32(&[Rev_A1(AL, R::R0, R::R1)]),
            a: 0,
            b: pat,
            expected: pat.swap_bytes(),
        },
        Case {
            name: "A32 rev16 r0,r1",
            thumb: false,
            bytes: a32(&[Rev16_A1(AL, R::R0, R::R1)]),
            a: 0,
            b: pat,
            expected: ((pat >> 8) & 0x00FF_00FF) | ((pat << 8) & 0xFF00_FF00),
        },
        Case {
            name: "A32 clz r0,r1",
            thumb: false,
            bytes: a32(&[Clz_A1(AL, R::R0, R::R1)]),
            a: 0,
            b: 0x0000_FFFF,
            expected: 0x0000_FFFFu32.leading_zeros(),
        },
    ];

    let mut c = String::new();
    c.push_str("/* generated by arm32_hardware_exec_gen -- DO NOT EDIT */\n");
    c.push_str(
        "#include <stdint.h>\n#include <stdio.h>\n#include <string.h>\n#include <sys/mman.h>\n\n",
    );
    c.push_str("typedef uint32_t (*fn2)(uint32_t, uint32_t);\n\n");
    c.push_str("static uint32_t run(const unsigned char* code, size_t len, uint32_t a, uint32_t b, int thumb) {\n");
    c.push_str(
        "    void* m = mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);\n",
    );
    c.push_str("    if (m == MAP_FAILED) { perror(\"mmap\"); return (uint32_t)-1; }\n");
    c.push_str("    memcpy(m, code, len);\n");
    c.push_str("    if (mprotect(m, 4096, PROT_READ|PROT_EXEC) != 0) { perror(\"mprotect\"); return (uint32_t)-1; }\n");
    c.push_str("    __builtin___clear_cache((char*)m, (char*)m + len);\n");
    // Thumb entry needs bit0 set; ARM-state entry has it clear -> the indirect call interworks to A32.
    c.push_str("    uintptr_t entry = (uintptr_t)m; if (thumb) entry |= 1u;\n");
    c.push_str("    uint32_t r = ((fn2)entry)(a, b);\n");
    c.push_str("    munmap(m, 4096);\n    return r;\n}\n\n");
    c.push_str(
        "int main(void) {\n    setvbuf(stdout, NULL, _IONBF, 0);\n    int pass = 0, fail = 0;\n",
    );
    for case in &cases {
        let bytes: Vec<String> = case
            .bytes
            .iter()
            .map(|byte| format!("0x{byte:02x}"))
            .collect();
        c.push_str(&format!(
            "    {{ static const unsigned char code[] = {{{}}};\n      uint32_t got = run(code, sizeof code, 0x{:x}u, 0x{:x}u, {});\n      uint32_t exp = 0x{:x}u;\n      if (got == exp) pass++; else {{ printf(\"  FAIL %-34s got 0x%08x exp 0x%08x\\n\", \"{}\", got, exp); fail++; }} }}\n",
            bytes.join(", "), case.a, case.b, if case.thumb { 1 } else { 0 }, case.expected, case.name
        ));
    }
    c.push_str("    printf(\"hardware-exec on aarch32 (T32+A32): %d passed, %d failed\\n\", pass, fail);\n");
    c.push_str("    return fail ? 1 : 0;\n}\n");

    std::fs::write("arm32_hardware_exec.c", &c).expect("write arm32_hardware_exec.c");
    eprintln!(
        "wrote arm32_hardware_exec.c ({} cases, {} bytes)",
        cases.len(),
        c.len()
    );
}
