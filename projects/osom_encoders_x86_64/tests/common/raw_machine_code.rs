use osom_encoders_x86_64::models::EncodedX86_64Instruction;

pub struct RawMachineCode {
    code: region::Allocation,
    length: usize,
}

impl RawMachineCode {
    pub fn from_code(code: &[EncodedX86_64Instruction]) -> Self {
        let prot = region::Protection::READ_WRITE_EXECUTE;
        let pz = core::cmp::max(4096, region::page::size());
        let alloc = region::alloc(pz, prot).unwrap();
        let mut result = Self { code: alloc, length: 0 };
        for instruction in code {
            result.write_instruction(instruction);
        }
        result
    }

    fn write_instruction(&mut self, instruction: &EncodedX86_64Instruction) {
        let slice = instruction.as_slice();
        if self.length + slice.len() > self.code.len() {
            let new_len = self.code.len() * 2;
            let mut new_alloc = region::alloc(new_len, region::Protection::READ_WRITE_EXECUTE).unwrap();
            let src_ptr: *const u8 = self.code.as_ptr();
            let dst_ptr: *mut u8 = new_alloc.as_mut_ptr();
            unsafe { dst_ptr.copy_from_nonoverlapping(src_ptr, self.length) };
            self.code = new_alloc;
        }

        unsafe {
            let dst_ptr = self.code.as_mut_ptr::<u8>().add(self.length);
            std::ptr::copy_nonoverlapping(slice.as_ptr(), dst_ptr, slice.len());
        }
        self.length += slice.len();
    }

    pub fn raw_ptr(&self, offset: u32) -> *const u8 {
        unsafe { self.code.as_ptr::<u8>().add(offset as usize) }
    }
}

#[macro_export]
macro_rules! compile {
    ( $rmc: expr, $offset: expr, $abi: literal ) => {
        unsafe {
            let ptr = $rmc.raw_ptr($offset);
            std::mem::transmute::<*const u8, unsafe extern $abi fn() -> u64>(ptr)
        }
    };

    ( $rmc: expr, $offset: expr, $abi: literal, ($arg: ident : $arg_type: ty) ) => {
        unsafe {
            let ptr = $rmc.raw_ptr($offset);
            std::mem::transmute::<*const u8, unsafe extern $abi fn($arg: $arg_type) -> u64>(ptr)
        }
    };

    ( $rmc: expr, $offset: expr, $abi: literal, ($arg1: ident : $arg_type1: ty, $arg2: ident : $arg_type2: ty) ) => {
        unsafe {
            let ptr = $rmc.raw_ptr($offset);
            std::mem::transmute::<*const u8, unsafe extern $abi fn($arg1: $arg_type1, $arg2: $arg_type2) -> u64>(ptr)
        }
    };
}

pub use compile;
