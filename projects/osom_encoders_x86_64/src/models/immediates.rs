/// Represents an 8-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate8 {
    value: u8,
}

impl Immediate8 {
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i8(value: i8) -> Self {
        Self {
            value: unsafe { core::mem::transmute::<i8, u8>(value) },
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 1] {
        [self.value]
    }
}

/// Represents a 16-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate16 {
    value: u16,
}

impl Immediate16 {
    #[inline(always)]
    pub const fn from_u16(value: u16) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i16(value: i16) -> Self {
        Self {
            value: unsafe { core::mem::transmute::<i16, u16>(value) },
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 2] {
        self.value.to_le_bytes()
    }
}

/// Represents a 32-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate32 {
    value: u32,
}

impl Immediate32 {
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i32(value: i32) -> Self {
        Self {
            value: unsafe { core::mem::transmute::<i32, u32>(value) },
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 4] {
        self.value.to_le_bytes()
    }
}

/// Represents a 64-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate64 {
    value: u64,
}

impl Immediate64 {
    #[inline(always)]
    pub const fn from_u64(value: u64) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i64(value: i64) -> Self {
        Self {
            value: unsafe { core::mem::transmute::<i64, u64>(value) },
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn value(&self) -> u64 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 8] {
        self.value.to_le_bytes()
    }
}
