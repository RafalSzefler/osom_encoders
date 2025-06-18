#![cfg(target_arch = "x86_64")]
#![allow(unused_imports)]
#![allow(unused_unsafe)]

mod common;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

use crate::common::raw_machine_code::RawMachineCode;

#[test]
fn test_execution_ret() {
    unsafe {
        let rmc = RawMachineCode::from_code(&[ret::encode_ret()]);
        let func = compile!(rmc, 0, "sysv64");
        let _ = func();
    }
}

#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
#[case(5)]
#[case(6)]
#[case(7)]
#[case(8)]
#[case(9)]
fn test_execution_nop(#[case] nop_length: u8) {
    unsafe {
        const VAL: i8 = 17;
        let rmc = RawMachineCode::from_code(&[
            miscellaneous::encode_nop_with_length(nop_length),
            mov::encode_mov_rm8_imm8(GPROrMemory::GPR { gpr: GPR::AL }, Immediate8::from_i8(VAL)),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64");
        let call_result = func();
        let result = std::mem::transmute::<u64, i64>(call_result) as i8;
        assert_eq!(result, VAL);
    }
}

#[rstest]
#[case(-128)]
#[case(-77)]
#[case(-1)]
#[case(0)]
#[case(1)]
#[case(77)]
#[case(127)]
fn test_execution_mov_8(#[case] val: i8) {
    unsafe {
        let rmc = RawMachineCode::from_code(&[
            mov::encode_mov_rm8_imm8(GPROrMemory::GPR { gpr: GPR::AL }, Immediate8::from_i8(val)),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64");
        let call_result = func();
        let result = std::mem::transmute::<u64, i64>(call_result) as i8;
        assert_eq!(result, val);
    }
}

#[rstest]
#[case(-500)]
#[case(-128)]
#[case(-77)]
#[case(-1)]
#[case(0)]
#[case(1)]
#[case(77)]
#[case(127)]
#[case(500)]
fn test_execution_mov_16(#[case] val: i16) {
    unsafe {
        let rmc = RawMachineCode::from_code(&[
            mov::encode_mov_rm16_imm16(GPROrMemory::GPR { gpr: GPR::AX }, Immediate16::from_i16(val)),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64");
        let call_result = func();
        let result = std::mem::transmute::<u64, i64>(call_result) as i16;
        assert_eq!(result, val);
    }
}

#[rstest]
#[case(-6321432)]
#[case(-500)]
#[case(-128)]
#[case(-77)]
#[case(-1)]
#[case(0)]
#[case(1)]
#[case(77)]
#[case(127)]
#[case(500)]
#[case(6321432)]
fn test_execution_mov_32(#[case] val: i32) {
    unsafe {
        let rmc = RawMachineCode::from_code(&[
            mov::encode_mov_rm32_imm32(GPROrMemory::GPR { gpr: GPR::EAX }, Immediate32::from_i32(val)),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64");
        let call_result = func();
        let result = std::mem::transmute::<u64, i64>(call_result) as i32;
        assert_eq!(result, val);
    }
}

#[rstest]
#[case(-6321432)]
#[case(-500)]
#[case(-128)]
#[case(-77)]
#[case(-1)]
#[case(0)]
#[case(1)]
#[case(77)]
#[case(127)]
#[case(500)]
#[case(6321432)]
fn test_execution_mov_64(#[case] val: i32) {
    unsafe {
        let rmc = RawMachineCode::from_code(&[
            mov::encode_mov_rm64_imm32(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(val)),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64");
        let call_result = func();
        let result = std::mem::transmute::<u64, i64>(call_result) as i32;
        assert_eq!(result, val);
    }
}

#[rstest]
#[case(-21474836423811)]
#[case(-6321432)]
#[case(-500)]
#[case(-128)]
#[case(-77)]
#[case(-1)]
#[case(0)]
#[case(1)]
#[case(77)]
#[case(127)]
#[case(500)]
#[case(6321432)]
#[case(21474836423811)]
fn test_execution_mov_64_imm64(#[case] val: i64) {
    unsafe {
        let rmc = RawMachineCode::from_code(&[
            mov::encode_mov_reg64_imm64(GPR::RAX, Immediate64::from_i64(val)),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64");
        let call_result = func() as i64;
        assert_eq!(call_result, val);
    }
}

#[test]
fn test_execution_jmp() {
    unsafe {
        let ret_instr = ret::encode_ret();
        let ret_len = ret_instr.as_slice().len() as i8;

        let mov_instr = mov::encode_mov_rm64_imm32(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(1));
        let mov_len = mov_instr.as_slice().len() as i8;

        let rel8 = -(ret_len + mov_len + 2);

        let rmc = RawMachineCode::from_code(&[
            ret_instr.clone(),
            mov_instr,
            jmp::encode_jmp_imm8(Immediate8::from_i8(rel8)),
            mov::encode_mov_rm64_imm32(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(2)),
            ret_instr,
        ]);

        // The second argument is the offset of the return instruction. Meaning we start at
        // the mov instruction.
        let func = compile!(rmc, ret_len as u32, "sysv64");
        let call_result = func() as i64;
        assert_eq!(call_result, 1);
    }
}

#[rstest]
#[case(-1000, 0)]
#[case(-2, 0)]
#[case(-1, 0)]
#[case(0, 1)]
#[case(1, 1)]
#[case(2, 1)]
#[case(1000, 1)]
fn test_execution_jcc_and_cmp(#[case] arg: i64, #[case] expected: i64) {
    unsafe {
        // According to sysv64 calling convention, the first argument is in RDI.
        // While the return value is in RAX.
        let rmc = RawMachineCode::from_code(&[
            mov::encode_mov_rm64_imm32(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(1)),
            cmp::encode_cmp_rm64_imm32(GPROrMemory::GPR { gpr: GPR::RDI }, Immediate32::from_i32(0)),
            jcc::encode_jcc_GE_imm8(Immediate8::from_i8(7)), // We know that the next instruction takes 7 bytes, jump over it.
            mov::encode_mov_rm64_imm32(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(0)),
            ret::encode_ret(),
        ]);

        // The second argument is the offset of the return instruction. Meaning we start at
        // the mov instruction.
        let func = compile!(rmc, 0, "sysv64", (arg: i64));
        let call_result = func(arg) as i64;
        assert_eq!(call_result, expected);
    }
}

#[rstest]
#[case(1, 2, 3)]
#[case(-1, 1, 0)]
#[case(1, -1, 0)]
#[case(1, 1, 2)]
#[case(-1, -1, -2)]
#[case(0, 0, 0)]
#[case(123, 456, 579)]
#[case(-123, -456, -579)]
#[case(123, -456, -333)]
#[case(-123, 456, 333)]
fn test_execution_sum_through_lea(#[case] a: i64, #[case] b: i64, #[case] expected: i64) {
    unsafe {
        // According to sysv64 calling convention, the first argument is in RDI, second RSI.
        // While the return value is in RAX.
        let rmc = RawMachineCode::from_code(&[
            lea::encode_lea_reg64_m(
                GPR::RAX,
                Memory::BasedScaled {
                    base: GPR::RDI,
                    index: GPR::RSI,
                    scale: Scale::Scale1,
                    offset: Offset::None,
                },
            ),
            ret::encode_ret(),
        ]);
        let func = compile!(rmc, 0, "sysv64", (a: i64, b: i64));
        let call_result = func(a, b) as i64;
        assert_eq!(call_result, expected);
    }
}
