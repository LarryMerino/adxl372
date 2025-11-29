//! High-level ADXL372 device driver implementation.

use crate::config::Config;
use crate::error::{Error, Result};
use crate::interface::Adxl372Interface;

/// High-level synchronous driver for the ADXL372 accelerometer.
pub struct Adxl372<IFACE> {
    interface: IFACE,
    config: Config,
}

impl<IFACE> Adxl372<IFACE> {
    /// Creates a new driver instance from the provided bus interface.
    pub fn new(interface: IFACE, config: Config) -> Self {
        Self { interface, config }
    }

    /// Consumes the driver and returns the owned interface.
    pub fn release(self) -> (IFACE, Config) {
        (self.interface, self.config)
    }
}

impl<IFACE, CommE> Adxl372<IFACE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    /// Initializes the sensor using the current configuration.
    pub fn init(&mut self) -> Result<(), CommE> {
        self.config.validate().map_err(|_| Error::InvalidConfig)?;
        Ok(())
    }

    /// Applies a new configuration to the device.
    pub fn configure(&mut self, config: Config) -> Result<(), CommE> {
        config.validate().map_err(|_| Error::InvalidConfig)?;
        self.config = config;
        Ok(())
    }

    /// Returns a shared reference to the active configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns a mutable reference to the active configuration.
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}
