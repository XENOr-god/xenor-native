use core::fmt;
use std::ptr::NonNull;

use crate::checksum::Checksum;
use crate::ffi;
use crate::snapshot::{InputFrame, SimulationSeed, StateSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    CreationFailed,
    NativeStatus(i32),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreationFailed => f.write_str("failed to allocate native simulation"),
            Self::NativeStatus(status) => write!(f, "native runtime returned status {}", status),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub struct SimulationRuntime {
    raw: NonNull<ffi::XenorSimulation>,
}

impl SimulationRuntime {
    pub fn new(seed: SimulationSeed) -> Result<Self, RuntimeError> {
        let raw = unsafe { ffi::xenor_sim_create(seed.into()) };
        let raw = NonNull::new(raw).ok_or(RuntimeError::CreationFailed)?;
        Ok(Self { raw })
    }

    pub fn reset(&mut self, seed: SimulationSeed) -> Result<(), RuntimeError> {
        Self::check_status(unsafe { ffi::xenor_sim_reset(self.raw.as_ptr(), seed.into()) })
    }

    pub fn step(&mut self, input: InputFrame) -> Result<StateSnapshot, RuntimeError> {
        let frame = input.into_ffi();
        Self::check_status(unsafe { ffi::xenor_sim_step(self.raw.as_ptr(), &frame) })?;
        self.snapshot()
    }

    pub fn run(&mut self, inputs: &[InputFrame]) -> Result<StateSnapshot, RuntimeError> {
        for input in inputs {
            self.step(*input)?;
        }

        self.snapshot()
    }

    pub fn snapshot(&self) -> Result<StateSnapshot, RuntimeError> {
        let mut snapshot = ffi::XenorStateSnapshot::default();
        Self::check_status(unsafe { ffi::xenor_sim_snapshot(self.raw.as_ptr(), &mut snapshot) })?;
        Ok(snapshot.into())
    }

    pub fn checksum(&self) -> Result<Checksum, RuntimeError> {
        let mut checksum = ffi::XenorChecksum::default();
        Self::check_status(unsafe { ffi::xenor_sim_checksum(self.raw.as_ptr(), &mut checksum) })?;
        Ok(checksum.into())
    }

    pub fn tick(&self) -> u64 {
        unsafe { ffi::xenor_sim_tick(self.raw.as_ptr()) }
    }

    fn check_status(status: i32) -> Result<(), RuntimeError> {
        if status == ffi::XENOR_STATUS_OK {
            Ok(())
        } else {
            Err(RuntimeError::NativeStatus(status))
        }
    }
}

impl Drop for SimulationRuntime {
    fn drop(&mut self) {
        unsafe { ffi::xenor_sim_destroy(self.raw.as_ptr()) };
    }
}
