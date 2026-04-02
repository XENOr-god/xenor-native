use core::ffi::c_int;

pub(crate) const XENOR_STATUS_OK: c_int = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct XenorSeed {
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct XenorInputFrame {
    pub throttle: u32,
    pub steer: i32,
    pub action_mask: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct XenorStateSnapshot {
    pub tick: u64,
    pub position: i64,
    pub velocity: i64,
    pub energy: u64,
    pub accumulator: u64,
    pub last_input_mix: u64,
    pub phase: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct XenorChecksum {
    pub value: u64,
}

#[repr(C)]
pub(crate) struct XenorSimulation {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub(crate) fn xenor_sim_create(seed: XenorSeed) -> *mut XenorSimulation;
    pub(crate) fn xenor_sim_destroy(simulation: *mut XenorSimulation);
    pub(crate) fn xenor_sim_reset(simulation: *mut XenorSimulation, seed: XenorSeed) -> c_int;
    pub(crate) fn xenor_sim_step(
        simulation: *mut XenorSimulation,
        input: *const XenorInputFrame,
    ) -> c_int;
    pub(crate) fn xenor_sim_snapshot(
        simulation: *const XenorSimulation,
        out_snapshot: *mut XenorStateSnapshot,
    ) -> c_int;
    pub(crate) fn xenor_sim_checksum(
        simulation: *const XenorSimulation,
        out_checksum: *mut XenorChecksum,
    ) -> c_int;
    pub(crate) fn xenor_sim_tick(simulation: *const XenorSimulation) -> u64;
}
