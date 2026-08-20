use modular_bitfield::prelude::*;

use crate::params::{ClockSource, OutputDataRate, SyncMode, WakeUpRate};

/// Base trait implemented by every ADXL372 register
pub trait Register {
    const ADDRESS: u8;
}

/// Marker trait for registers that can be read.
pub trait ReadableRegister: Register {}

/// Marker trait for registers that can be written.
pub trait WritableRegister: Register {}

pub struct Timing {
    pub odr: OutputDataRate,
    pub wakeup_rate: WakeUpRate,
    pub clock_source: ClockSource,
    pub sync_mode: SyncMode,
}

impl Register for Timing {
    const ADDRESS: u8 = 0x3D;
}

#[bitfield]
struct TimingBits {
    odr: B3,
    wakeup_rate: B3,
    ext_clk: bool,
    ext_sync: bool,
}

pub struct Reset;

impl Register for Reset {
    const ADDRESS: u8 = 0x41;
}

impl WritableRegister for Reset {}

impl Reset {
    pub const COMMAND: u8 = 0x52;
}