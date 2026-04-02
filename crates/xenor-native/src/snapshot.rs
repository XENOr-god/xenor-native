use core::fmt;

use crate::ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SimulationSeed(pub u64);

impl SimulationSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<SimulationSeed> for ffi::XenorSeed {
    fn from(seed: SimulationSeed) -> Self {
        Self { value: seed.0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct InputFrame {
    pub throttle: u32,
    pub steer: i32,
    pub action_mask: u8,
}

impl InputFrame {
    pub const fn new(throttle: u32, steer: i32, action_mask: u8) -> Self {
        Self {
            throttle,
            steer,
            action_mask,
        }
    }

    pub(crate) const fn into_ffi(self) -> ffi::XenorInputFrame {
        ffi::XenorInputFrame {
            throttle: self.throttle,
            steer: self.steer,
            action_mask: self.action_mask,
            reserved: [0; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    Input,
    Simulate,
    Finalize,
}

impl Phase {
    pub(crate) const fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::Input,
            1 => Self::Simulate,
            _ => Self::Finalize,
        }
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Input => "input",
            Self::Simulate => "simulate",
            Self::Finalize => "finalize",
        };
        f.write_str(name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateSnapshot {
    pub tick: u64,
    pub phase: Phase,
    pub position: i64,
    pub velocity: i64,
    pub energy: u64,
    pub accumulator: u64,
    pub last_input_mix: u64,
    pub flags: u32,
}

impl StateSnapshot {
    pub fn summary(&self) -> String {
        format!(
            "tick={} phase={} position={} velocity={} energy={} accumulator={:016x} input_mix={:016x} flags=0x{:x}",
            self.tick,
            self.phase,
            self.position,
            self.velocity,
            self.energy,
            self.accumulator,
            self.last_input_mix,
            self.flags
        )
    }
}

impl From<ffi::XenorStateSnapshot> for StateSnapshot {
    fn from(snapshot: ffi::XenorStateSnapshot) -> Self {
        Self {
            tick: snapshot.tick,
            phase: Phase::from_raw(snapshot.phase),
            position: snapshot.position,
            velocity: snapshot.velocity,
            energy: snapshot.energy,
            accumulator: snapshot.accumulator,
            last_input_mix: snapshot.last_input_mix,
            flags: snapshot.flags,
        }
    }
}
