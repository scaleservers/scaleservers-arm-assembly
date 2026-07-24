// Copyright (c) Scaleservers LLC

// UAL (Unified Assembly Language) emitter for Arm64Instruction. See emit.rs for the entry point.
//
// House notes on the rendering choices (chosen so a differential oracle can compare against the real
// assemblers/disassemblers when one is installed):
//   * registers render lowercase as their canonical `x`-names; the `31` encoding renders `xzr` or `sp`
//     per the variant in the model;
//   * objdump-preferred aliases ARE rendered (see `preferred_alias`): `ORR Xd, XZR, Xm` -> `mov`,
//     `SUBS XZR, Rn, Rm` -> `cmp`, `UBFM` -> `lsl`/`lsr`/`ubfx`/..., etc.; non-alias forms render their
//     architectural mnemonic;
//   * immediates print per `ArmAssemblySyntax` -- decimal `#N` (GNU) or hex `#0xN` (LLVM);
//   * PC-relative branch operands print as signed byte offsets (e.g. `.+8` / `.-4`), since without an
//     address context there is no absolute target to resolve.

use alloc::string::{String, ToString};

use crate::arm64_instruction::Arm64Instruction;
use crate::emit::ArmAssemblySyntax;
use crate::enums::{
    Arm64FloatPrecision, Arm64LoadStoreIndex, Arm64LoadStoreSize, Arm64RegisterWidth,
};

// Render an immediate in the selected flavor: decimal for GNU, hex (`0x...`) for LLVM. Negative values get a
// leading `-` before the radix prefix.
fn imm(value: i64, syntax: ArmAssemblySyntax) -> String {
    match syntax {
        ArmAssemblySyntax::Gnu => format!("#{value}"),
        ArmAssemblySyntax::Llvm => {
            if value < 0 {
                format!("#-0x{:x}", value.unsigned_abs())
            } else {
                format!("#0x{value:x}")
            }
        }
    }
}

// Render a PC-relative byte offset as a label-relative target (`.`, `.+N`, `.-N`).
// The 16 canonical FEAT_MOPS copy option suffixes, indexed by `RN<<3 | WN<<2 | RT<<1 | WT` (the [15:12] option
// field, read/write non-temporal/unprivileged). Captured from `llvm-mc -disassemble` of all 16 combos.
const MOPS_COPY_SUFFIX: [&str; 16] = [
    "", "wt", "rt", "t", "wn", "wtwn", "rtwn", "twn", "rn", "wtrn", "rtrn", "trn", "n", "wtn",
    "rtn", "tn",
];

fn rel(offset: i32) -> String {
    match offset.cmp(&0) {
        core::cmp::Ordering::Equal => ".".to_string(),
        core::cmp::Ordering::Greater => format!(".+{offset}"),
        core::cmp::Ordering::Less => format!(".-{}", offset.unsigned_abs()),
    }
}

// The i64 form (ADRP's page byte offset spans +/-4 GiB, beyond i32).
fn rel64(offset: i64) -> String {
    match offset.cmp(&0) {
        core::cmp::Ordering::Equal => ".".to_string(),
        core::cmp::Ordering::Greater => format!(".+{offset}"),
        core::cmp::Ordering::Less => format!(".-{}", offset.unsigned_abs()),
    }
}


// Render an SVE INDEX base/step operand: `#imm`, or the scalar register at `Wn` (`.b`/`.h`/`.s`) / `Xn` (`.d`).
fn sve_index_operand(
    operand: &crate::enums::Arm64SveIndexOperand,
    size: crate::enums::Arm64VectorElement,
) -> alloc::string::String {
    match operand {
        crate::enums::Arm64SveIndexOperand::Immediate(v) => format!("#{v}"),
        crate::enums::Arm64SveIndexOperand::Register(r) => {
            let width = if matches!(size, crate::enums::Arm64VectorElement::D) {
                Arm64RegisterWidth::X
            } else {
                Arm64RegisterWidth::W
            };
            r.name_for_width(width).to_string()
        }
    }
}

// Render an SVE `[Xn|SP{, #imm, MUL VL}]` addressing operand: a `#0` offset is omitted, otherwise the element-count
// The strided multi-vector register list: `{Zn, Zn+8}` (vgx2) or `{Zn, Zn+4, Zn+8, Zn+12}` (vgx4).
fn sme2_strided_list(base: u8, four: bool, el: &str) -> alloc::string::String {
    if four {
        format!(
            "{{z{base}.{el}, z{}.{el}, z{}.{el}, z{}.{el}}}",
            base + 4,
            base + 8,
            base + 12
        )
    } else {
        format!("{{z{base}.{el}, z{}.{el}}}", base + 8)
    }
}

// The mnemonic prefix for an SME2 multi-vector contiguous load/store (the `LDNT1`/`STNT1` non-temporal forms add `nt`).
fn multivec_ldst_prefix(store: bool, non_temporal: bool) -> &'static str {
    match (store, non_temporal) {
        (false, false) => "ld",
        (false, true) => "ldnt",
        (true, false) => "st",
        (true, true) => "stnt",
    }
}

// The mnemonic size letter for an SME2 multi-vector contiguous load/store (`ld1w`/`st1w` for a `.s` access; the other
// sizes match the element letter).
fn multivec_msz_letter(msz: crate::enums::Arm64VectorElement) -> &'static str {
    use crate::enums::Arm64VectorElement::{B, H, S};
    match msz {
        B => "b",
        H => "h",
        S => "w",
        _ => "d",
    }
}

// offset prints as `#<imm>, mul vl`.
fn sve_mul_vl_addr(
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    imm: i32,
) -> alloc::string::String {
    if imm == 0 {
        format!("[{}]", rn.ual_name())
    } else {
        format!("[{}, #{imm}, mul vl]", rn.ual_name())
    }
}

// `[Xn, Xm{, lsl #shift}]` -- an SVE scalar-base, scalar-index address. The index is scaled by the access size;
// the `lsl #shift` is shown only when it actually shifts (a byte access prints the bare register pair).
fn sve_scalar_index_addr(
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    rm: &crate::enums::Arm64GeneralPurposeRegister,
    shift: u32,
) -> alloc::string::String {
    if shift == 0 {
        format!(
            "[{}, {}]",
            rn.ual_name(),
            rm.name_for_width(Arm64RegisterWidth::X)
        )
    } else {
        format!(
            "[{}, {}, lsl #{shift}]",
            rn.ual_name(),
            rm.name_for_width(Arm64RegisterWidth::X)
        )
    }
}

// Derive the element size and per-element value of an SVE bitwise-immediate repeating bitmask, for UAL display.
// The full 64-bit pattern always repeats with some power-of-two period; we pick the smallest so the printed
// `.<T>` and `#imm` match the architectural disassembly (e.g. `0x000000ff000000ff` -> `.s, #0xff`).
fn sve_bitmask_element(imm: u64) -> (crate::enums::Arm64VectorElement, u64) {
    use crate::enums::Arm64VectorElement;
    for (bits, elem) in [
        (8u32, Arm64VectorElement::B),
        (16, Arm64VectorElement::H),
        (32, Arm64VectorElement::S),
    ] {
        let lo = imm & ((1u64 << bits) - 1);
        let mut tiled = 0u64;
        let mut shift = 0;
        while shift < 64 {
            tiled |= lo << shift;
            shift += bits;
        }
        if tiled == imm {
            return (elem, lo);
        }
    }
    (Arm64VectorElement::D, imm)
}

// Render the optional `{, <pattern>{, MUL #imm}}` tail shared by the SVE element-count instructions: empty when the
// pattern is `ALL` and `mul` is 1; `, <pattern>` when only the pattern is non-default; `, <pattern>, mul #imm` when
// `mul != 1` (the pattern then always prints, defaulting to `all`).
fn sve_count_tail(pattern: u8, mul: u8) -> alloc::string::String {
    if mul != 1 {
        let pat = sve_pattern_name(pattern).unwrap_or_else(|| "all".to_string());
        format!(", {pat}, mul #{mul}")
    } else {
        match sve_pattern_name(pattern) {
            Some(pat) => format!(", {pat}"),
            None => alloc::string::String::new(),
        }
    }
}

// Render an SVE SIMD&FP scalar destination (`CLASTA`/`LASTA` etc.): `b0`/`h0`/`s0`/`d0`, the element-size view of
// the register. (The shared [`Arm64FloatRegister::name_for_precision`] has no byte view, so map the element here.)
fn sve_simd_scalar_name(
    reg: crate::enums::Arm64FloatRegister,
    element: crate::enums::Arm64VectorElement,
) -> alloc::string::String {
    use crate::enums::Arm64VectorElement::{B, D, H, S};
    let letter = match element {
        B => "b",
        H => "h",
        S => "s",
        D => "d",
    };
    format!("{letter}{}", reg.as_operand_bits())
}

// The element-count mnemonic letter for `CNT`/`INC`/`DEC` (b/h/w/d) -- the access-size letter, so `.s` -> `w`.
fn sve_count_letter(element: crate::enums::Arm64VectorElement) -> &'static str {
    match element {
        crate::enums::Arm64VectorElement::B => "b",
        crate::enums::Arm64VectorElement::H => "h",
        crate::enums::Arm64VectorElement::S => "w",
        crate::enums::Arm64VectorElement::D => "d",
    }
}

// Render an SVE predicate-count pattern (the `<pattern>` operand of `PTRUE` etc.): `pow2`, `vl1`..`vl256`, `mul4`,
// `mul3`, or `#n` for a reserved value. The `ALL` pattern (31) returns `None` (it is the default, printed bare).
fn sve_pattern_name(pattern: u8) -> Option<alloc::string::String> {
    Some(match pattern {
        0 => "pow2".to_string(),
        1..=8 => format!("vl{pattern}"),
        9 => "vl16".to_string(),
        10 => "vl32".to_string(),
        11 => "vl64".to_string(),
        12 => "vl128".to_string(),
        13 => "vl256".to_string(),
        29 => "mul4".to_string(),
        30 => "mul3".to_string(),
        31 => return None,
        other => format!("#{other}"),
    })
}

// Render a data-processing (1 source) instruction: `mnem Rd, Rn`, both in the variant width.
fn render_dp1(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    format!(
        "{} {}, {}",
        mnemonic,
        xd.name_for_width(width),
        xn.name_for_width(width)
    )
}

// Render a scalar FP data-processing (3 source) instruction: `mnem Fd, Fn, Fm, Fa`. All operands render in
// the variant precision (`s`/`d`).
fn render_float_3source(
    mnemonic: &str,
    precision: Arm64FloatPrecision,
    fd: &crate::enums::Arm64FloatRegister,
    fn_: &crate::enums::Arm64FloatRegister,
    fm: &crate::enums::Arm64FloatRegister,
    fa: &crate::enums::Arm64FloatRegister,
) -> String {
    format!(
        "{} {}, {}, {}, {}",
        mnemonic,
        fd.name_for_precision(precision),
        fn_.name_for_precision(precision),
        fm.name_for_precision(precision),
        fa.name_for_precision(precision)
    )
}

// The FP precision paired with a GP width for the bit-preserving FMOV GP<->FP move: `X -> Double`, `W -> Single`.
fn fp_precision_for_width(width: Arm64RegisterWidth) -> Arm64FloatPrecision {
    match width {
        Arm64RegisterWidth::X => Arm64FloatPrecision::Double,
        Arm64RegisterWidth::W => Arm64FloatPrecision::Single,
    }
}

// Render a scalar FP data-processing (2 source) instruction: `mnem Fd, Fn, Fm`. All three operands render in
// the variant precision (`s`/`d`); the form carries no immediate, so it's syntax-independent.
fn render_float_2source(
    mnemonic: &str,
    precision: Arm64FloatPrecision,
    fd: &crate::enums::Arm64FloatRegister,
    fn_: &crate::enums::Arm64FloatRegister,
    fm: &crate::enums::Arm64FloatRegister,
) -> String {
    format!(
        "{} {}, {}, {}",
        mnemonic,
        fd.name_for_precision(precision),
        fn_.name_for_precision(precision),
        fm.name_for_precision(precision)
    )
}

// Render a scalar FP data-processing (1 source) instruction: `mnem Fd, Fn`. Both operands render in the
// variant precision (`s`/`d`). Used by FNEG/FABS/FSQRT and the FP->FP register FMOV.
fn render_float_1source(
    mnemonic: &str,
    precision: Arm64FloatPrecision,
    fd: &crate::enums::Arm64FloatRegister,
    fn_: &crate::enums::Arm64FloatRegister,
) -> String {
    format!(
        "{} {}, {}",
        mnemonic,
        fd.name_for_precision(precision),
        fn_.name_for_precision(precision)
    )
}

fn render_addsub_immediate(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    imm12: u16,
    shift12: bool,
    syntax: ArmAssemblySyntax,
) -> String {
    // add/sub-immediate is the SP-positioned family: field 31 renders as the stack pointer (`sp`/`wsp`).
    let mut text = format!(
        "{} {}, {}, {}",
        mnemonic,
        xd.name_for_width_sp(width),
        xn.name_for_width_sp(width),
        imm(imm12 as i64, syntax)
    );
    if shift12 {
        text.push_str(", lsl ");
        text.push_str(&imm(12, syntax));
    }
    text
}

// ADDS/SUBS (immediate): the dest is a real result (or XZR for the CMN/CMP aliases), so `Rd` renders in the ZR
// view (`wzr`/`xzr`) while `Rn` stays SP-positioned (`sp`/`wsp`).
fn render_addsub_immediate_flagset(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    imm12: u16,
    shift12: bool,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!(
        "{} {}, {}, {}",
        mnemonic,
        xd.name_for_width(width),
        xn.name_for_width_sp(width),
        imm(imm12 as i64, syntax)
    );
    if shift12 {
        text.push_str(", lsl ");
        text.push_str(&imm(12, syntax));
    }
    text
}

fn render_move_wide(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    imm16: u16,
    hw: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!(
        "{} {}, {}",
        mnemonic,
        xd.name_for_width(width),
        imm(imm16 as i64, syntax)
    );
    if hw != 0 {
        text.push_str(", lsl ");
        text.push_str(&imm((hw as i64) * 16, syntax));
    }
    text
}

fn render_shifted_register(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
    amount: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!(
        "{} {}, {}, {}",
        mnemonic,
        xd.name_for_width(width),
        xn.name_for_width(width),
        xm.name_for_width(width)
    );
    if amount != 0 {
        text.push_str(", lsl ");
        text.push_str(&imm(amount as i64, syntax));
    }
    text
}

// Render an add/subtract (extended register): `mnem Rd, Rn, Rm, <extend>[ #amount]`. `Rd` renders in the SP
// view for ADD/SUB (`rd_is_sp`) and the ZR view for the flag-setting ADDS/SUBS; `Rn` always renders in the SP
// view; `Rm` renders in the width the option selects (W for the sub-word extends, X for UXTX/SXTX). The extend
// operator is always printed explicitly, with `#amount` omitted when zero -- matching what GNU disassembles for
// the non-aliased forms, and a form both GNU and LLVM accept and encode identically to the `lsl`/bare aliases.
fn render_addsub_extended(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    rm: &crate::enums::Arm64GeneralPurposeRegister,
    option: crate::enums::Arm64ExtendOption,
    amount: u8,
    rd_is_sp: bool,
    syntax: ArmAssemblySyntax,
) -> String {
    let rd_name = if rd_is_sp {
        rd.name_for_width_sp(width)
    } else {
        rd.name_for_width(width)
    };
    let rn_name = rn.name_for_width_sp(width);
    let rm_name = rm.name_for_width(option.source_width());
    let mut text = format!(
        "{} {}, {}, {}, {}",
        mnemonic,
        rd_name,
        rn_name,
        rm_name,
        option.name()
    );
    if amount != 0 {
        text.push(' ');
        text.push_str(&imm(amount as i64, syntax));
    }
    text
}

// Render a logical (immediate): `mnem Rd, Rn, #0x<imm>`. AND/ORR/EOR render `Rd` in the SP view (`rd_is_sp`);
// ANDS renders the ZR view. `Rn` is always the ZR view. The bitmask immediate is printed as a hex constant (the
// form GNU and LLVM both emit and accept), not the raw `(N, immr, imms)` fields.
fn render_logical_immediate(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    imm: u64,
    rd_is_sp: bool,
) -> String {
    let rd_name = if rd_is_sp {
        rd.name_for_width_sp(width)
    } else {
        rd.name_for_width(width)
    };
    format!(
        "{} {}, {}, #0x{:x}",
        mnemonic,
        rd_name,
        rn.name_for_width(width),
        imm
    )
}

// Render a data-processing (2 source) instruction: `mnem Rd, Rn, Rm`. These forms carry no immediate or
// shift modifier, so rendering is syntax-independent; the registers render in the given width (`w`/`x`).
fn render_two_source(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    format!(
        "{} {}, {}, {}",
        mnemonic,
        xd.name_for_width(width),
        xn.name_for_width(width),
        xm.name_for_width(width)
    )
}

// Render a data-processing (3 source) instruction with an addend: `mnem Rd, Rn, Rm, Ra` (MADD/MSUB). Like
// the 2-source forms it carries no immediate/shift, so rendering is syntax-independent; width-aware.
fn render_three_source(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
    xa: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    format!(
        "{} {}, {}, {}, {}",
        mnemonic,
        xd.name_for_width(width),
        xn.name_for_width(width),
        xm.name_for_width(width),
        xa.name_for_width(width)
    )
}

// Render a 3-source long-multiply instruction: `mnem Xd, Wn, Wm, Xa` -- Rd/Ra are 64-bit (X), Rn/Rm are 32-bit (W).
fn render_long_multiply(
    mnemonic: &str,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
    xa: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    use Arm64RegisterWidth::{W, X};
    format!(
        "{} {}, {}, {}, {}",
        mnemonic,
        xd.name_for_width(X),
        xn.name_for_width(W),
        xm.name_for_width(W),
        xa.name_for_width(X)
    )
}

// ===================== alias-rendering helpers (see `preferred_alias`) =====================

// The constant a MOVZ/MOVN wide-immediate move materializes: `imm16 << (hw*16)`, masked to the register
// width, then bit-inverted for MOVN. Used when MOVZ/MOVN render as the `mov Rd, #imm` alias.
fn mov_wide_value(width: Arm64RegisterWidth, imm16: u16, hw: u8, invert: bool) -> u64 {
    let raw = (imm16 as u64) << (hw as u64 * 16);
    let mask = if width == Arm64RegisterWidth::W { 0xFFFF_FFFF } else { u64::MAX };
    let value = raw & mask;
    if invert { !value & mask } else { value }
}

// Whether a MOVZ/MOVN would be the preferred disassembly of this bitmask-immediate value (so an
// `ORR Rd, ZR, #imm` should NOT render as the `mov Rd, #imm` alias). Re-encodes the value to `(N, immr, imms)`
// and applies the ARM ARM `MoveWidePreferred` predicate (DDI0487).
fn move_wide_preferred(width: Arm64RegisterWidth, value: u64) -> bool {
    let reg_size = if width == Arm64RegisterWidth::X { 64 } else { 32 };
    match crate::bitmask_immediate::encode_bitmask(value, reg_size) {
        Some((n, immr, imms)) => move_wide_preferred_fields(width == Arm64RegisterWidth::X, n, imms, immr),
        None => false,
    }
}

fn move_wide_preferred_fields(sf: bool, n: u32, imms: u32, immr: u32) -> bool {
    let width = if sf { 64i32 } else { 32 };
    let s = imms as i32;
    let r = immr as i32;
    if sf && n != 1 {
        return false;
    }
    if !(sf || n == 0 && (imms & 0b10_0000) == 0) {
        return false;
    }
    if s < 16 {
        return (-r).rem_euclid(16) <= 15 - s;
    }
    if s >= width - 15 {
        return r % 16 <= s - (width - 15);
    }
    false
}

// CMP/CMN alias of SUBS/ADDS (immediate): `mnem Rn, #imm{, lsl #12}`. `Rn` renders SP-positioned.
fn addsub_imm_alias(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    imm12: u16,
    shift12: bool,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!("{} {}, {}", mnemonic, rn.name_for_width_sp(width), imm(imm12 as i64, syntax));
    if shift12 {
        text.push_str(", lsl ");
        text.push_str(&imm(12, syntax));
    }
    text
}

// CMP/CMN/TST alias of SUBS/ADDS/ANDS (shifted register): `mnem Rn, Rm{, lsl #amount}`. ZR-view registers.
fn shift_two(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    rm: &crate::enums::Arm64GeneralPurposeRegister,
    amount: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!("{} {}, {}", mnemonic, rn.name_for_width(width), rm.name_for_width(width));
    if amount != 0 {
        text.push_str(", lsl ");
        text.push_str(&imm(amount as i64, syntax));
    }
    text
}

// CMP/CMN alias of SUBS/ADDS (extended register): `mnem Rn, Rm, <extend>[ #amount]`. `Rn` SP-positioned; `Rm`
// in the extend's source width. Mirrors `render_addsub_extended` minus the destination.
fn ext_two(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    rm: &crate::enums::Arm64GeneralPurposeRegister,
    option: crate::enums::Arm64ExtendOption,
    amount: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!(
        "{} {}, {}, {}",
        mnemonic,
        rn.name_for_width_sp(width),
        rm.name_for_width(option.source_width()),
        option.name()
    );
    if amount != 0 {
        text.push(' ');
        text.push_str(&imm(amount as i64, syntax));
    }
    text
}

// NEG/NEGS/MVN alias of SUB/SUBS/ORN (Rn == ZR): `mnem Rd, Rm{, lsl #amount}`. ZR-view registers.
fn neg_two(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rm: &crate::enums::Arm64GeneralPurposeRegister,
    amount: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    let mut text = format!("{} {}, {}", mnemonic, rd.name_for_width(width), rm.name_for_width(width));
    if amount != 0 {
        text.push_str(", lsl ");
        text.push_str(&imm(amount as i64, syntax));
    }
    text
}

// SMULL/UMULL/SMNEGL/UMNEGL alias of SMADDL/... (Ra == ZR): `mnem Xd, Wn, Wm`.
fn long_mul_alias(
    mnemonic: &str,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    rm: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    use Arm64RegisterWidth::{W, X};
    format!("{} {}, {}, {}", mnemonic, rd.name_for_width(X), rn.name_for_width(W), rm.name_for_width(W))
}

// The preferred alias of UBFM (unsigned bitfield move) -- every UBFM is disassembled as one of these.
fn ubfm_alias(
    width: Arm64RegisterWidth,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    immr: u8,
    imms: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    use Arm64RegisterWidth::W;
    let bits: u32 = if width == Arm64RegisterWidth::X { 64 } else { 32 };
    let (immr, imms) = (immr as u32, imms as u32);
    let rd_s = rd.name_for_width(width);
    let rn_s = rn.name_for_width(width);
    if imms == bits - 1 {
        return format!("lsr {}, {}, {}", rd_s, rn_s, imm(immr as i64, syntax));
    }
    if imms + 1 == immr {
        return format!("lsl {}, {}, {}", rd_s, rn_s, imm((bits - 1 - imms) as i64, syntax));
    }
    if width == W && immr == 0 && imms == 7 {
        return format!("uxtb {}, {}", rd_s, rn_s);
    }
    if width == W && immr == 0 && imms == 15 {
        return format!("uxth {}, {}", rd_s, rn_s);
    }
    if imms < immr {
        return format!(
            "ubfiz {}, {}, {}, {}",
            rd_s, rn_s, imm(((bits - immr) % bits) as i64, syntax), imm((imms + 1) as i64, syntax)
        );
    }
    format!("ubfx {}, {}, {}, {}", rd_s, rn_s, imm(immr as i64, syntax), imm((imms - immr + 1) as i64, syntax))
}

// The preferred alias of SBFM (signed bitfield move). The SXT* source always renders as a `w` register.
fn sbfm_alias(
    width: Arm64RegisterWidth,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    immr: u8,
    imms: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    use Arm64RegisterWidth::{W, X};
    let bits: u32 = if width == X { 64 } else { 32 };
    let (immr, imms) = (immr as u32, imms as u32);
    let rd_s = rd.name_for_width(width);
    let rn_s = rn.name_for_width(width);
    let rn_w = rn.name_for_width(W);
    if imms == bits - 1 {
        return format!("asr {}, {}, {}", rd_s, rn_s, imm(immr as i64, syntax));
    }
    if immr == 0 {
        if imms == 7 {
            return format!("sxtb {}, {}", rd_s, rn_w);
        }
        if imms == 15 {
            return format!("sxth {}, {}", rd_s, rn_w);
        }
        if imms == 31 && width == X {
            return format!("sxtw {}, {}", rd_s, rn_w);
        }
    }
    if imms < immr {
        return format!(
            "sbfiz {}, {}, {}, {}",
            rd_s, rn_s, imm(((bits - immr) % bits) as i64, syntax), imm((imms + 1) as i64, syntax)
        );
    }
    format!("sbfx {}, {}, {}, {}", rd_s, rn_s, imm(immr as i64, syntax), imm((imms - immr + 1) as i64, syntax))
}

// The preferred alias of BFM (bitfield move): BFI (insert; `imms < immr`) or BFXIL (extract-into-low). Note
// the reference disassemblers render `BFM Rd, ZR, ...` as `bfi Rd, zr, ...` -- the pre-BFC spelling -- rather
// than the newer `BFC` alias, so we match that (validated by tests/alias_parity.rs).
fn bfm_alias(
    width: Arm64RegisterWidth,
    rd: &crate::enums::Arm64GeneralPurposeRegister,
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    immr: u8,
    imms: u8,
    syntax: ArmAssemblySyntax,
) -> String {
    let bits: u32 = if width == Arm64RegisterWidth::X { 64 } else { 32 };
    let (immr, imms) = (immr as u32, imms as u32);
    let rd_s = rd.name_for_width(width);
    let rn_s = rn.name_for_width(width);
    if imms < immr {
        let lsb = imm(((bits - immr) % bits) as i64, syntax);
        let bfwidth = imm((imms + 1) as i64, syntax);
        return format!("bfi {}, {}, {}, {}", rd_s, rn_s, lsb, bfwidth);
    }
    format!("bfxil {}, {}, {}, {}", rd_s, rn_s, imm(immr as i64, syntax), imm((imms - immr + 1) as i64, syntax))
}

// Render a conditional-select instruction: `mnem Rd, Rn, Rm, cond` (e.g. `csel x0, x1, x2, eq`). No immediate,
// so rendering is syntax-independent; width-aware. The condition prints as its bare lowercase suffix (e.g.
// `eq`) -- NOT the dotted `.eq` form, which is only used by branches like `b.eq`.
fn render_conditional_select(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    xd: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
    cond: crate::enums::Arm64Condition,
) -> String {
    format!(
        "{} {}, {}, {}, {}",
        mnemonic,
        xd.name_for_width(width),
        xn.name_for_width(width),
        xm.name_for_width(width),
        cond.ual_suffix()
    )
}

// The size suffix for a single-register load/store mnemonic: `b` for Byte, `h` for Half, none for Word/Double
// (there the `w`/`x` register width already disambiguates). So `ldr`+Byte => `ldrb`, `str`+Half => `strh`,
// `ldr`+Word => `ldr` (of a `w` register), `ldr`+Double => `ldr` (of an `x` register).
fn ldr_mnemonic(stem: &str, size: Arm64LoadStoreSize) -> String {
    match size {
        Arm64LoadStoreSize::Byte => format!("{stem}b"),
        Arm64LoadStoreSize::Half => format!("{stem}h"),
        Arm64LoadStoreSize::Word | Arm64LoadStoreSize::Double => stem.to_string(),
    }
}

// Render a single-register load/store (unsigned-immediate offset): `mnem Rt, [Rn{, #imm}]`. `Rt` renders in
// the size-derived width (`w` for Byte/Half/Word, `x` for Double); `Rn` renders as the stack pointer (`sp`) at
// field 31. A `#0` offset is omitted to match the toolchains' disassembly.
fn render_ldst_register(
    mnemonic: String,
    size: Arm64LoadStoreSize,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset_bytes: u32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = xt.name_for_width(size.rt_width());
    let base = ldst_base_name(xn);
    if offset_bytes == 0 {
        format!("{mnemonic} {rt}, [{base}]")
    } else {
        format!(
            "{mnemonic} {rt}, [{base}, {}]",
            imm(offset_bytes as i64, syntax)
        )
    }
}

// Render a load/store register-offset `mnem Rt, [Xn, Rm{, <ext> #amount}]`. `rt_width` is Rt's view (the access
// size's `rt_width` for LDR/STR, the explicit dest width for the signed loads); `shift_amount` is
// `log2(access_size)`, shown only when the `S` bit (`scaled`) is set. The modifier is omitted entirely for the
// plain `[Xn, Xm]` (LSL, unscaled) form; otherwise the extend operator is printed (`lsl`/`uxtw`/`sxtw`/`sxtx`),
// with `#amount` appended when scaled. `Rm` renders in the width the extend selects.
fn render_ldst_register_offset(
    mnemonic: &str,
    rt_width: Arm64RegisterWidth,
    shift_amount: u32,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
    extend: crate::enums::Arm64MemoryExtend,
    scaled: bool,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = xt.name_for_width(rt_width);
    let base = ldst_base_name(xn);
    let rm = xm.name_for_width(extend.index_width());
    let mut address = format!("[{base}, {rm}");
    if extend != crate::enums::Arm64MemoryExtend::Lsl || scaled {
        address.push_str(", ");
        address.push_str(extend.name());
        if scaled {
            address.push(' ');
            address.push_str(&imm(shift_amount as i64, syntax));
        }
    }
    address.push(']');
    format!("{mnemonic} {rt}, {address}")
}

// Pick the mnemonic stem for a 9-bit-immediate load/store by addressing mode: the `unscaled` stem for LDUR/STUR,
// the `unprivileged` stem for LDTR/STTR, else the `indexed` (plain LDR/STR) stem for pre/post-index.
fn imm9_stem(
    mode: crate::enums::Arm64Imm9Mode,
    unscaled: &'static str,
    unprivileged: &'static str,
    indexed: &'static str,
) -> &'static str {
    match mode {
        crate::enums::Arm64Imm9Mode::Unscaled => unscaled,
        crate::enums::Arm64Imm9Mode::Unprivileged => unprivileged,
        _ => indexed,
    }
}

// Render a load/store with the 9-bit unscaled immediate. The address form follows the mode: Unscaled/Unprivileged
// print `[Xn{, #imm}]` (a `#0` omitted), PostIndex `[Xn], #imm`, PreIndex `[Xn, #imm]!`. `Rt` renders in `rt_width`
// (the access size's view for LDUR/STUR, the explicit dest width for the signed loads).
fn render_ldst_imm9(
    mnemonic: &str,
    rt_width: Arm64RegisterWidth,
    mode: crate::enums::Arm64Imm9Mode,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset: i32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = xt.name_for_width(rt_width);
    let base = ldst_base_name(xn);
    let off = imm(offset as i64, syntax);
    let address = match mode {
        crate::enums::Arm64Imm9Mode::Unscaled | crate::enums::Arm64Imm9Mode::Unprivileged => {
            if offset == 0 {
                format!("[{base}]")
            } else {
                format!("[{base}, {off}]")
            }
        }
        crate::enums::Arm64Imm9Mode::PostIndex => format!("[{base}], {off}"),
        crate::enums::Arm64Imm9Mode::PreIndex => format!("[{base}, {off}]!"),
    };
    format!("{mnemonic} {rt}, {address}")
}

// Render a test-and-branch (`tbz`/`tbnz`): `mnem Rt, #<bit>, <target>`. `Rt` renders as `W` when the tested bit
// is in the low word (`bit < 32`), else `X`; the bit position prints as a plain decimal `#<bit>` and the target
// as the PC-relative `.+/-<offset>`.
fn render_test_branch(
    mnemonic: &str,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    bit: u8,
    offset: i32,
) -> String {
    let width = if bit >= 32 {
        Arm64RegisterWidth::X
    } else {
        Arm64RegisterWidth::W
    };
    format!(
        "{mnemonic} {}, #{bit}, {}",
        xt.name_for_width(width),
        rel(offset)
    )
}

// Render a NEON "three same" lane-arithmetic instruction: `mnem Vd.<arr>, Vn.<arr>, Vm.<arr>` -- all three vector
// operands carry the same arrangement suffix (e.g. `add v0.4s, v1.4s, v2.4s`).
fn render_vec_three(
    mnemonic: &str,
    arrangement: crate::enums::Arm64VectorArrangement,
    rd: &crate::enums::Arm64FloatRegister,
    rn: &crate::enums::Arm64FloatRegister,
    rm: &crate::enums::Arm64FloatRegister,
) -> String {
    let arr = arrangement.name();
    format!(
        "{mnemonic} v{}.{arr}, v{}.{arr}, v{}.{arr}",
        rd.as_operand_bits(),
        rn.as_operand_bits(),
        rm.as_operand_bits()
    )
}

// Render a NEON two-register-misc instruction: `mnem Vd.<arr>, Vn.<arr>` -- both operands carry the same
// arrangement suffix (e.g. `abs v0.4s, v1.4s`).
// Render a SIMD&FP single-register load/store `mnem <Vt>, [Xn{, #imm}]`. The transfer register is named with
// the access-size letter (`b`/`h`/`s`/`d`/`q`) via the size enum; `Xn` renders as the stack pointer (`sp`) at
// field 31; a `#0` offset is omitted to match the toolchains' disassembly.
fn render_vec_ldst(
    mnemonic: &str,
    size: crate::enums::Arm64VectorLoadStoreSize,
    vt: &crate::enums::Arm64FloatRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset_bytes: u32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = size.register_name(vt);
    let base = ldst_base_name(xn);
    if offset_bytes == 0 {
        format!("{mnemonic} {rt}, [{base}]")
    } else {
        format!(
            "{mnemonic} {rt}, [{base}, {}]",
            imm(offset_bytes as i64, syntax)
        )
    }
}

// Decode an SVE FDUP/FCPY 8-bit FP immediate back to its float value string (the shortest round-tripping form,
// as GNU/LLVM print it): the `.d` element uses the double decode, `.h`/`.s` the single decode. Mirrors the scalar
// FMOV-immediate rendering so the emitted `#<value>` re-assembles to the same imm8.
fn sve_fp_imm_value(size: crate::enums::Arm64VectorElement, imm8: u8) -> alloc::string::String {
    if matches!(size, crate::enums::Arm64VectorElement::D) {
        format!("{:?}", crate::fp8_decode_double(imm8))
    } else {
        format!("{:?}", crate::fp8_decode_single(imm8))
    }
}

// Render a SIMD&FP load/store pair `mnem <Vt>, <Vt2>, <addr>` where the address form follows the index mode
// (Offset `[Xn{, #imm}]` with a `#0` omitted, PreIndex `[Xn, #imm]!`, PostIndex `[Xn], #imm`). The transfer
// registers are named with the access-size letter (`s`/`d`/`q`); `Xn` renders as `sp` at field 31.
fn render_vec_ldst_pair(
    mnemonic: &str,
    index: Arm64LoadStoreIndex,
    size: crate::enums::Arm64VectorLoadStoreSize,
    vt: &crate::enums::Arm64FloatRegister,
    vt2: &crate::enums::Arm64FloatRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset_bytes: i32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = size.register_name(vt);
    let rt2 = size.register_name(vt2);
    let base = ldst_base_name(xn);
    let address = match index {
        Arm64LoadStoreIndex::Offset => {
            if offset_bytes == 0 {
                format!("[{base}]")
            } else {
                format!("[{base}, {}]", imm(offset_bytes as i64, syntax))
            }
        }
        Arm64LoadStoreIndex::PreIndex => format!("[{base}, {}]!", imm(offset_bytes as i64, syntax)),
        Arm64LoadStoreIndex::PostIndex => format!("[{base}], {}", imm(offset_bytes as i64, syntax)),
    };
    format!("{mnemonic} {rt}, {rt2}, {address}")
}

// Render a SIMD&FP load/store register-offset `mnem <Vt>, [Xn, Rm{, <ext> #amount}]`. `Vt` is named with the
// access-size letter; the index `Rm` renders in the extend's width, and the `<ext> #amount` modifier shows when
// the extend is not the plain `LSL` or when the `S` bit (`scaled`) shifts the index by `log2(access_size)`.
fn render_vec_ldst_register_offset(
    mnemonic: &str,
    size: crate::enums::Arm64VectorLoadStoreSize,
    vt: &crate::enums::Arm64FloatRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    xm: &crate::enums::Arm64GeneralPurposeRegister,
    extend: crate::enums::Arm64MemoryExtend,
    scaled: bool,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = size.register_name(vt);
    let base = ldst_base_name(xn);
    let rm = xm.name_for_width(extend.index_width());
    let mut address = format!("[{base}, {rm}");
    if extend != crate::enums::Arm64MemoryExtend::Lsl || scaled {
        address.push_str(", ");
        address.push_str(extend.name());
        if scaled {
            address.push(' ');
            address.push_str(&imm(size.scale() as i64, syntax));
        }
    }
    address.push(']');
    format!("{mnemonic} {rt}, {address}")
}

// Render a SIMD&FP load/store with the 9-bit unscaled immediate. Unscaled prints `ldur`/`stur ... [Xn{, #imm}]`
// (a `#0` omitted), pre/post-index the plain `ldr`/`str` with `[Xn, #imm]!` / `[Xn], #imm`. `Vt` is named with
// the access-size letter. (SIMD&FP has no unprivileged form, so that arm never renders.)
fn render_vec_ldst_imm9(
    mnemonic: &str,
    mode: crate::enums::Arm64Imm9Mode,
    size: crate::enums::Arm64VectorLoadStoreSize,
    vt: &crate::enums::Arm64FloatRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset: i32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = size.register_name(vt);
    let base = ldst_base_name(xn);
    let off = imm(offset as i64, syntax);
    let address = match mode {
        crate::enums::Arm64Imm9Mode::Unscaled | crate::enums::Arm64Imm9Mode::Unprivileged => {
            if offset == 0 {
                format!("[{base}]")
            } else {
                format!("[{base}, {off}]")
            }
        }
        crate::enums::Arm64Imm9Mode::PostIndex => format!("[{base}], {off}"),
        crate::enums::Arm64Imm9Mode::PreIndex => format!("[{base}, {off}]!"),
    };
    format!("{mnemonic} {rt}, {address}")
}

// Render a NEON three-different op `mnem{2} Vd.<Ta>, Vn.<Tb>, Vm.<Tc>`. `wide` is the 128-bit side
// (`.8h`/`.4s`/`.2d`); `high` appends the `2` upper-half suffix and selects the 128-bit narrow arrangement. The
// op's shape fixes which operands take the wide vs the narrow suffix (Long: Vd wide; Wide: Vd,Vn wide; Narrow: Vd
// narrow).
fn render_vec_three_different(
    op: crate::enums::Arm64VectorThreeDifferentOp,
    wide: crate::enums::Arm64VectorArrangement,
    high: bool,
    rd: &crate::enums::Arm64FloatRegister,
    rn: &crate::enums::Arm64FloatRegister,
    rm: &crate::enums::Arm64FloatRegister,
) -> String {
    use crate::enums::Arm64VectorThreeDifferentShape::{Long, Narrow, Wide};
    let suffix = if high { "2" } else { "" };
    let wide_s = wide.name();
    let narrow = crate::enums::Arm64VectorArrangement::from_q_and_size(
        if high { 1 } else { 0 },
        wide.size_bits() - 1,
    );
    let narrow_s = narrow.name();
    let (ta, tb, tc) = match op.shape() {
        Long => (wide_s, narrow_s, narrow_s),
        Wide => (wide_s, wide_s, narrow_s),
        Narrow => (narrow_s, wide_s, wide_s),
    };
    format!(
        "{}{} v{}.{}, v{}.{}, v{}.{}",
        op.name(),
        suffix,
        rd.as_operand_bits(),
        ta,
        rn.as_operand_bits(),
        tb,
        rm.as_operand_bits(),
        tc,
    )
}

// Render a NEON long/narrowing shift `mnem{2} Vd.<Ta>, Vn.<Tb>, #<shift>`. `narrow` is the narrow side (its Q
// bit gives the `2` upper-half suffix); the wide side is twice as wide. For the long (widening) ops Vd is wide
// and Vn narrow; for the narrowing ops Vd is narrow and Vn wide.
fn render_vec_shift_long_narrow(
    op: crate::enums::Arm64VectorShiftLongNarrowOp,
    narrow: crate::enums::Arm64VectorArrangement,
    rd: &crate::enums::Arm64FloatRegister,
    rn: &crate::enums::Arm64FloatRegister,
    shift: u8,
) -> String {
    let suffix = if narrow.q_bit() == 1 { "2" } else { "" };
    let narrow_s = narrow.name();
    let wide_s =
        crate::enums::Arm64VectorArrangement::from_q_and_size(1, narrow.size_bits() + 1).name();
    let (ta, tb) = if op.is_long() {
        (wide_s, narrow_s)
    } else {
        (narrow_s, wide_s)
    };
    format!(
        "{}{} v{}.{}, v{}.{}, #{}",
        op.name(),
        suffix,
        rd.as_operand_bits(),
        ta,
        rn.as_operand_bits(),
        tb,
        shift
    )
}

// Render a NEON across-lanes reduction `mnem <V>d, Vn.<arr>`. The destination is a scalar named with the result
// width's size letter (`b`/`h`/`s`/`d`); the source is the vector arrangement.
fn render_vec_across_lanes(
    op: crate::enums::Arm64VectorAcrossLanesOp,
    arrangement: crate::enums::Arm64VectorArrangement,
    rd: &crate::enums::Arm64FloatRegister,
    rn: &crate::enums::Arm64FloatRegister,
) -> String {
    let letter = ['b', 'h', 's', 'd'][op.result_size(arrangement) as usize];
    format!(
        "{} {}{}, v{}.{}",
        op.name(),
        letter,
        rd.as_operand_bits(),
        rn.as_operand_bits(),
        arrangement.name()
    )
}

// Render a SHA register operand in the given view (`q0` / `s0` / `v0.4s`).
fn render_sha_reg(
    view: crate::enums::Arm64ShaRegView,
    reg: &crate::enums::Arm64FloatRegister,
) -> String {
    use crate::enums::Arm64ShaRegView::{Q, S, V2d, V4s};
    let n = reg.as_operand_bits();
    match view {
        Q => format!("q{n}"),
        S => format!("s{n}"),
        V4s => format!("v{n}.4s"),
        V2d => format!("v{n}.2d"),
    }
}

fn render_vec_two(
    mnemonic: &str,
    arrangement: crate::enums::Arm64VectorArrangement,
    rd: &crate::enums::Arm64FloatRegister,
    rn: &crate::enums::Arm64FloatRegister,
) -> String {
    let arr = arrangement.name();
    format!(
        "{mnemonic} v{}.{arr}, v{}.{arr}",
        rd.as_operand_bits(),
        rn.as_operand_bits()
    )
}

// Render a NEON shift-by-immediate instruction: `mnem Vd.<arr>, Vn.<arr>, #<shift>`.
fn render_vec_shift_imm(
    mnemonic: &str,
    arrangement: crate::enums::Arm64VectorArrangement,
    rd: &crate::enums::Arm64FloatRegister,
    rn: &crate::enums::Arm64FloatRegister,
    shift: u8,
) -> String {
    let arr = arrangement.name();
    format!(
        "{mnemonic} v{}.{arr}, v{}.{arr}, #{shift}",
        rd.as_operand_bits(),
        rn.as_operand_bits()
    )
}

// The general-purpose register width a NEON lane move uses for a given element size: `.d` (size 3) pairs with
// an X register; every smaller element uses a W register.
fn gp_width_for_size(size: u32) -> crate::enums::Arm64RegisterWidth {
    if size == 3 {
        crate::enums::Arm64RegisterWidth::X
    } else {
        crate::enums::Arm64RegisterWidth::W
    }
}

// Render a two-operand atomic load/store (`ldxr`/`ldaxr`/`ldar`/`stlr`): `mnem Rt, [Xn]`. `Rt` renders in the
// access-size width; `Xn` is the base (SP view at field 31).
fn render_atomic_load(
    mnemonic: &str,
    rt_width: Arm64RegisterWidth,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    format!(
        "{mnemonic} {}, [{}]",
        xt.name_for_width(rt_width),
        ldst_base_name(xn)
    )
}

// Render a store-exclusive (`stxr`/`stlxr`): `mnem Ws, Rt, [Xn]`. The status register `Ws` is always 32-bit.
fn render_store_exclusive(
    mnemonic: &str,
    ws: &crate::enums::Arm64GeneralPurposeRegister,
    rt_width: Arm64RegisterWidth,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    format!(
        "{mnemonic} {}, {}, [{}]",
        ws.name_for_width(Arm64RegisterWidth::W),
        xt.name_for_width(rt_width),
        ldst_base_name(xn)
    )
}

// Render a CAS / LSE-RMW / SWP (`cas`/`ldadd`/`swp`/...): `mnem Rs, Rt, [Xn]`. Both data registers render in the
// access-size width.
fn render_atomic_rmw(
    mnemonic: &str,
    reg_width: Arm64RegisterWidth,
    rs: &crate::enums::Arm64GeneralPurposeRegister,
    rt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
) -> String {
    format!(
        "{mnemonic} {}, {}, [{}]",
        rs.name_for_width(reg_width),
        rt.name_for_width(reg_width),
        ldst_base_name(xn)
    )
}

// Render a signed load (`LDRSB`/`LDRSH`/`LDRSW`): `mnem Rt, [Rn{, #imm}]`. Unlike the unsigned loads, `Rt`
// renders in the explicit sign-extend destination width (`w`/`x`), not the access size; `Rn` renders as the
// stack pointer (`sp`) at field 31. A `#0` offset is omitted to match the toolchains' disassembly.
fn render_ldst_signed(
    mnemonic: &str,
    dest_width: Arm64RegisterWidth,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset_bytes: u32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = xt.name_for_width(dest_width);
    let base = ldst_base_name(xn);
    if offset_bytes == 0 {
        format!("{mnemonic} {rt}, [{base}]")
    } else {
        format!(
            "{mnemonic} {rt}, [{base}, {}]",
            imm(offset_bytes as i64, syntax)
        )
    }
}

// Render a load/store pair: `mnem Rt, Rt2, <addr>` where the address form follows the index mode:
//   Offset    => `[Rn{, #imm}]`   (a `#0` offset is omitted)
//   PreIndex  => `[Rn, #imm]!`
//   PostIndex => `[Rn], #imm`
// `Rt`/`Rt2` render in the variant width (`w`/`x`); `Rn` renders as `sp` at field 31. The signed offset prints
// per syntax flavor (decimal `#-16` for GNU, hex for LLVM).
fn render_ldst_pair(
    mnemonic: &str,
    width: Arm64RegisterWidth,
    index: Arm64LoadStoreIndex,
    xt: &crate::enums::Arm64GeneralPurposeRegister,
    xt2: &crate::enums::Arm64GeneralPurposeRegister,
    xn: &crate::enums::Arm64GeneralPurposeRegister,
    offset_bytes: i32,
    syntax: ArmAssemblySyntax,
) -> String {
    let rt = xt.name_for_width(width);
    let rt2 = xt2.name_for_width(width);
    let base = ldst_base_name(xn);
    let address = match index {
        Arm64LoadStoreIndex::Offset => {
            if offset_bytes == 0 {
                format!("[{base}]")
            } else {
                format!("[{base}, {}]", imm(offset_bytes as i64, syntax))
            }
        }
        Arm64LoadStoreIndex::PreIndex => format!("[{base}, {}]!", imm(offset_bytes as i64, syntax)),
        Arm64LoadStoreIndex::PostIndex => format!("[{base}], {}", imm(offset_bytes as i64, syntax)),
    };
    format!("{mnemonic} {rt}, {rt2}, {address}")
}

// The memory base register's name: the X view, but field 31 renders as the stack pointer `sp` (a load/store
// base is an SP-positioned operand, never the zero register). `ual_name` already maps the `Sp` variant to
// `sp` and the `Xzr` variant to `xzr`; the decoder always supplies `Sp` for field 31 here, so this prints
// `sp`. Plain `x0`..`x30` otherwise.
fn ldst_base_name(xn: &crate::enums::Arm64GeneralPurposeRegister) -> &'static str {
    xn.ual_name()
}

// Render the 5-bit PRFM prefetch operation as `<type><target><policy>` (e.g. `pldl1keep`), or `#n` for a reserved
// code (type 11 or target 11). type[4:3]: pld/pli/pst; target[2:1]: l1/l2/l3; policy[0]: keep/strm.
fn prefetch_op_name(prfop: u8) -> String {
    let type_name = match (prfop >> 3) & 0b11 {
        0 => "pld",
        1 => "pli",
        2 => "pst",
        _ => return format!("#{prfop}"),
    };
    let target_name = match (prfop >> 1) & 0b11 {
        0 => "l1",
        1 => "l2",
        2 => "l3",
        _ => return format!("#{prfop}"),
    };
    let policy_name = if prfop & 1 == 1 { "strm" } else { "keep" };
    format!("{type_name}{target_name}{policy_name}")
}

// Render the SME ZERO tile-select mask as the minimal `{...}` tile list, matching the assembler's greedy decomposition:
// `0xFF` -> `{za}`, then `.h` (4-slot, 0x55<<k), `.s` (2-slot, 0x11<<k), `.d` (1-slot, 1<<k) tiles in turn.
// Render an SME MOP4 source operand: a single vector `z{n}.{t}` or, for the list form, the 2-vector
// `{z{n}.{t}-z{n+1}.{t}}` (the even base + its successor).
fn sme_mop4_operand(base: u8, list: bool, elem: &str) -> alloc::string::String {
    if list {
        format!("{{z{base}.{elem}-z{}.{elem}}}", base + 1)
    } else {
        format!("z{base}.{elem}")
    }
}

fn sme_zero_tile_list(mask: u8) -> alloc::string::String {
    if mask == 0xFF {
        return alloc::string::String::from("{za}");
    }
    let mut remaining = mask;
    let mut tiles: alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();
    for k in 0..2u8 {
        let pattern = 0x55u8 << k;
        if remaining & pattern == pattern {
            tiles.push(format!("za{k}.h"));
            remaining &= !pattern;
        }
    }
    for k in 0..4u8 {
        let pattern = 0x11u8 << k;
        if remaining & pattern == pattern {
            tiles.push(format!("za{k}.s"));
            remaining &= !pattern;
        }
    }
    for k in 0..8u8 {
        let pattern = 1u8 << k;
        if remaining & pattern == pattern {
            tiles.push(format!("za{k}.d"));
            remaining &= !pattern;
        }
    }
    format!("{{{}}}", tiles.join(", "))
}

// Render the 4-bit SVE prefetch operation. Unlike the 5-bit A64 PRFM `prefetch_op_name`, the SVE form has only
// pld/pst (type = bit3) and no `pli`. target[2:1]: l1/l2/l3; policy[0]: keep/strm. Reserved codes print `#n`.
fn sve_prefetch_op_name(prfop: u8) -> alloc::string::String {
    let type_name = if prfop & 0b1000 == 0 { "pld" } else { "pst" };
    let target_name = match (prfop >> 1) & 0b11 {
        0 => "l1",
        1 => "l2",
        2 => "l3",
        _ => return format!("#{prfop}"),
    };
    let policy_name = if prfop & 1 == 1 { "strm" } else { "keep" };
    format!("{type_name}{target_name}{policy_name}")
}

// Render the addressing operand of an MTE tag load/store: `[Xn]` / `[Xn, #off]` / `[Xn, #off]!` (pre) / `[Xn], #off`
// (post).
fn render_tag_addr(
    rn: &crate::enums::Arm64GeneralPurposeRegister,
    index: crate::enums::Arm64LoadStoreIndex,
    offset: i32,
) -> String {
    use crate::enums::Arm64LoadStoreIndex::{Offset, PostIndex, PreIndex};
    let base = ldst_base_name(rn);
    match index {
        Offset if offset == 0 => format!("[{base}]"),
        Offset => format!("[{base}, #{offset}]"),
        PreIndex => format!("[{base}, #{offset}]!"),
        PostIndex => format!("[{base}], #{offset}"),
    }
}
