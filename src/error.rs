pub type DriverResult<T, IOE> = core::result::Result<T, Error<IOE>>;

pub enum Error<IOE> {
    Interface(IOE),
    InvalidConfig(ConfigError),
}

pub enum ConfigError {
    NyquistViolation,
    Int1ConflictWithExtClk,
}
