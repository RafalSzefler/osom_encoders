#![cfg(target_arch = "x86_64")]
use std::mem::transmute;

use _osom_encoders_x86_64_tests::ExecutableMemory;

use osom_encoders_x86_64::{
    encoders,
    models::{GPR, GPROrMemory, Memory, Scale},
};

macro_rules! to_fn {
    (fn($($args:ty),* $(,)?) $(-> $ret:ty)? ; $mem:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let ptr = ($mem).as_ptr();
            transmute::<*const u8, extern "sysv64" fn($($args),*) $(-> $ret)?>(ptr)
        }
    }};
}

#[test]
fn test_sysv64_ret() {
    unsafe {
        let mut memory = ExecutableMemory::new();
        memory.push(encoders::ret::encode());

        let exe = memory.as_exe();
        let func = to_fn!(fn(); exe);
        func();
    }
}

#[test]
fn test_sysv64_add() {
    unsafe {
        let mut memory = ExecutableMemory::new();
        let rax = GPROrMemory::from(GPR::RAX);
        memory.push(encoders::mov::encode_rm64_imm32(rax, 0.into()));
        memory.push(encoders::add::encode_rm64_reg64(rax, GPR::RDI));
        memory.push(encoders::add::encode_rm64_reg64(rax, GPR::RSI));
        memory.push(encoders::ret::encode());

        let exe = memory.as_exe();
        let func = to_fn!(fn(u64, u64) -> u64; exe);

        assert_eq!(func(1, 2), 3);
    }
}

#[test]
fn test_sysv64_max() {
    unsafe {
        let mut memory = ExecutableMemory::new();
        let rax = GPROrMemory::from(GPR::RAX);
        let final_set_rax = encoders::mov::encode_rm64_reg64(rax, GPR::RDI);
        let final_set_size = final_set_rax.as_slice().len() as u8;
        memory.push(encoders::mov::encode_rm64_reg64(rax, GPR::RSI));
        memory.push(encoders::cmp::encode_reg64_rm64(GPR::RDI, GPROrMemory::from(GPR::RSI)));
        memory.push(encoders::jcc::encode_b_imm8(final_set_size.into()));
        memory.push(final_set_rax);
        memory.push(encoders::ret::encode());

        let exe = memory.as_exe();
        let func = to_fn!(fn(u64, u64) -> u64; exe);
        assert_eq!(func(1, 2), 2);
        assert_eq!(func(2, 1), 2);
        assert_eq!(func(1, 1), 1);
        assert_eq!(func(123411, 361234), 361234);
        assert_eq!(func(12223411, 361234), 12223411);
    }
}

#[test]
fn test_sysv64_lea() {
    unsafe {
        let mut memory = ExecutableMemory::new();
        memory.push(encoders::mov::encode_rm64_imm32(GPR::RAX.into(), 0.into()));
        let src = Memory::BasedAndScaled {
            base: GPR::RDI,
            index: GPR::RSI,
            scale: Scale::Scale1,
            offset: 0.into(),
        };
        memory.push(encoders::lea::encode_reg64_mem64(GPR::RAX, src));
        memory.push(encoders::ret::encode());

        let exe = memory.as_exe();
        let func = to_fn!(fn(i64, i64) -> i64; exe);
        assert_eq!(func(1, 1), 2);
        assert_eq!(func(-1, 0), -1);
        assert_eq!(func(0, -1), -1);
        assert_eq!(func(-1, -1), -2);
        assert_eq!(func(-1, 1), 0);
        assert_eq!(func(-16, 123), 107);
        assert_eq!(func(123, -16), 107);
    }
}
