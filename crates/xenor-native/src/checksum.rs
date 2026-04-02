use core::fmt;

use crate::ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Checksum(pub u64);

impl Checksum {
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl From<ffi::XenorChecksum> for Checksum {
    fn from(checksum: ffi::XenorChecksum) -> Self {
        Self(checksum.value)
    }
}
