use modular_bitfield::prelude::*;

/// Base trait implemented by every ADXL372 register
pub(crate) trait Register {
    const ADDRESS: u8;

    type Value;
}

/// Marker trait for registers that can be read.
pub(crate) trait ReadableRegister: Register {
    fn from_raw(raw: u8) -> Self::Value;
}

/// Marker trait for registers that can be written.
pub(crate) trait WritableRegister: Register {
    fn into_raw(value: Self::Value) -> u8;
}

// ---------------------------------------------------------------------
// TIMING register
// ---------------------------------------------------------------------

pub(crate) struct TimingRegister;

#[bitfield]
pub(crate) struct TimingBits {
    pub(crate) ext_sync: bool,
    pub(crate) ext_clk: bool,
    pub(crate) wakeup_rate: B3,
    pub(crate) odr: B3,
}

impl Register for TimingRegister {
    const ADDRESS: u8 = 0x3D;

    type Value = TimingBits;
}

impl ReadableRegister for TimingRegister {
    fn from_raw(raw: u8) -> Self::Value {
        TimingBits::from_bytes([raw])
    }
}

impl WritableRegister for TimingRegister {
    fn into_raw(value: Self::Value) -> u8 {
        value.into_bytes()[0]
    }
}

// ---------------------------------------------------------------------
// RESET register
// ---------------------------------------------------------------------

pub(crate) struct ResetRegister;

pub(crate) enum ResetCommand {
    Reset,
}

impl Register for ResetRegister {
    const ADDRESS: u8 = 0x41;

    type Value = ResetCommand;
}

impl WritableRegister for ResetRegister {
    fn into_raw(value: Self::Value) -> u8 {
        match value {
            ResetCommand::Reset => 0x52,
        }
    }
}

// ---------------------------------------------------------------------
// MESUREMENT register
// ---------------------------------------------------------------------

pub(crate) struct MeasureRegister;

#[bitfield]
pub(crate) struct MeasureBits {
    pub(crate) bandwidth: B3,
    pub(crate) low_noise: bool,
    pub(crate) linkloop: B2,
    pub(crate) autosleep: bool,
    pub(crate) user_or_disable: bool,
}

impl Register for MeasureRegister {
    const ADDRESS: u8 = 0x3E;

    type Value = MeasureBits;
}

impl ReadableRegister for MeasureRegister {
    fn from_raw(raw: u8) -> Self::Value {
        MeasureBits::from_bytes([raw])
    }
}

impl WritableRegister for MeasureRegister {
    fn into_raw(value: Self::Value) -> u8 {
        value.into_bytes()[0]
    }
}

// ---------------------------------------------------------------------
// POWER CONTROL register
// ---------------------------------------------------------------------

pub(crate) struct PowerCTLRegister;

#[bitfield]
pub(crate) struct PowerCTLBits {
    pub(crate) mode: B2,
    pub(crate) hpf_disable: bool,
    pub(crate) lpf_disable: bool,
    pub(crate) filter_settle: bool,
    pub(crate) instant_on_thresh: bool,
    #[skip]
    __: B1,
    pub(crate) i2c_hsm_en: bool,
}

impl Register for PowerCTLRegister {
    const ADDRESS: u8 = 0x3F;

    type Value = PowerCTLBits;
}

impl ReadableRegister for PowerCTLRegister {
    fn from_raw(raw: u8) -> Self::Value {
        PowerCTLBits::from_bytes([raw])
    }
}

impl WritableRegister for PowerCTLRegister {
    fn into_raw(value: Self::Value) -> u8 {
        value.into_bytes()[0]
    }
}
