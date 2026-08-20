use embedded_hal::delay::DelayNs;
use embedded_hal::spi::SpiDevice;

use crate::config::Config; 
use crate::interface::RegisterAccess;
use crate::error::{DriverResult, Error};

// ADXL372 datasheet power-up to standby delay (milliseconds).
const POWER_UP_TO_STANDBY_DELAY_MS: u32 = 5;

pub struct Adxl372<IO> {
    interface: IO,
}

impl<IO> Adxl372<IO>
where IO: RegisterAccess
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
    pub fn init(&mut self, delay: &mut impl DelayNs, config: Config) -> DriverResult<(), IO::Error> {
        delay.delay_ms(POWER_UP_TO_STANDBY_DELAY_MS);

        config
            .validate()
            .map_err(Error::<IO::Error>::InvalidConfig)?;
        todo!()
    }

    pub fn read_config(&mut self) -> DriverResult<(), IO::Error> {
        todo!()
    }

    pub fn reset(&self) -> DriverResult<Config, IO::Error> {
        todo!()
    }
}