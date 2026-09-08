pub type DriverResult<T, IOE> = core::result::Result<T, Error<IOE>>;

pub enum Error<IOE> {
    Interface(IOE),
    InvalidConfig(ConfigError),
    InvalidRegisterValue { address: u8, value: u8 },
}

pub enum ConfigError {
    NyquistViolation,
    Int1ConflictWithExtClk,
}
