use modular_bitfield::prelude::*;

use crate::params::{ClockSource, OutputDataRate, SyncMode, WakeUpRate};

pub const REG_TIMING: u8 = 0x3D;
pub const REG_RESET: u8 = 0x41;

pub enum RegisterAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

pub trait Register {
    const ADDRESS: u8;
    const ACCESS: RegisterAccess;
    const RESET_VALUE: Option<u8>;
}

pub struct Timing {
    pub odr: OutputDataRate,
    pub wakeup_rate: WakeUpRate,
    pub clock_source: ClockSource,
    pub sync_mode: SyncMode,
}

impl Register for Timing {
    const ADDRESS: u8 = REG_TIMING;
    const ACCESS: RegisterAccess = RegisterAccess::ReadWrite;
    const RESET_VALUE: Option<u8> = Some(0x00);
}

#[bitfield]
struct TimingBits {
    odr: B3,
    wakeup_rate: B3,
    ext_clk: bool,
    ext_sync: bool,
}


/// Soft reset command value written to the `RESET` register.
pub const RESET_COMMAND: u8 = 0x52;