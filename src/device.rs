use embedded_hal::delay::DelayNs;

use crate::config::Config;
use crate::error::{DriverResult, Error};
use crate::interface::RegisterAccess;
use crate::mapping::RegisterConfig;
use crate::registers::{
    MeasureRegister, PowerCTLRegister, ReadableRegister, ResetCommand, ResetRegister,
    TimingRegister, WritableRegister,
};

// ADXL372 datasheet power-up to standby delay (milliseconds).
const POWER_UP_TO_STANDBY_DELAY_MS: u32 = 5;

pub struct Adxl372<IO> {
    interface: IO,
}

impl<IO> Adxl372<IO>
where
    IO: RegisterAccess,
{
    pub fn new(interface: IO) -> Self {
        Self { interface }
    }

    /// Initializes the sensor using the current configuration.
    ///
    /// Enforces the datasheet power-up-to-standby delay before issuing any commands so callers
    /// do not need to provide their own wait after reset or power ramp.
    ///
    /// This initialization sequence runs the ER001 self-test prior to applying configuration.
    pub fn init(
        &mut self,
        delay: &mut impl DelayNs,
        config: Config,
    ) -> DriverResult<(), IO::Error> {
        delay.delay_ms(POWER_UP_TO_STANDBY_DELAY_MS);

        config
            .validate()
            .map_err(Error::<IO::Error>::InvalidConfig)?;
        todo!()
    }

    pub fn read_config(&mut self) -> DriverResult<Config, IO::Error> {

        let reg_conf = RegisterConfig {
            timing: self.read_register::<TimingRegister>()?,
            measure: self.read_register::<MeasureRegister>()?,
            power: self.read_register::<PowerCTLRegister>()?
        };

        Config::try_from(reg_conf)
            .map_err(Error::InvalidConfig)

        
    }

    pub fn reset(&mut self) -> DriverResult<(), IO::Error> {
        self.write_register::<ResetRegister>(ResetCommand::Reset)
    }

    fn write_register<R>(&mut self, value: R::Value) -> DriverResult<(), IO::Error>
    where
        R: WritableRegister,
    {
        let raw = R::encode(value);

        self.interface
            .write_register(R::ADDRESS, raw)
            .map_err(Error::Interface)
    }

    fn read_register<R>(&mut self) -> DriverResult<R::Value, IO::Error>
    where
        R: ReadableRegister,
    {
        let raw = self
            .interface
            .read_register(R::ADDRESS)
            .map_err(Error::Interface)?;

        R::decode(raw).ok_or(Error::InvalidRegisterValue {
            address: R::ADDRESS,
            value: raw,
        })
    }
}
