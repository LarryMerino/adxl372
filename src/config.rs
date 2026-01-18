//! Configuration primitives for the ADXL372 driver.

use crate::params::{Bandwidth, ExtClk, ExtSync, OutputDataRate, WakeUpRate};

/// User-facing configuration for the ADXL372 sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Output data rate selection.
    pub odr: OutputDataRate,
    /// Analog bandwidth selection.
    pub bandwidth: Bandwidth,
    /// Wake-up timer period used when autosleep/link-loop is enabled.
    pub wakeup_rate: Option<WakeUpRate>,
    /// External reference clock enable.
    pub ext_clk: Option<ExtClk>,
    /// External sync/trigger enable.
    pub ext_sync: Option<ExtSync>,
}

impl Config {
    /// Begins building a [`Config`] using the builder pattern.
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Checks whether this configuration is valid according to datasheet rules.
    pub fn validate(&self) -> core::result::Result<(), ConfigError> {
        if self.bandwidth.max_hz() * 2 > self.odr.hz() {
            return Err(ConfigError::NyquistViolation);
        }

        Ok(())
    }
}

/// Builder for [`Config`] allowing piecemeal construction.
#[derive(Debug, Clone, Copy)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Creates a new builder seeded with [`Config::default()`].
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Overrides the output data rate.
    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.config.odr = odr;
        self
    }

    /// Overrides the analog bandwidth.
    pub fn bandwidth(mut self, bandwidth: Bandwidth) -> Self {
        self.config.bandwidth = bandwidth;
        self
    }

    /// Sets the wake-up timer period.
    pub fn wakeup_rate(mut self, wakeup_rate: WakeUpRate) -> Self {
        self.config.wakeup_rate = Some(wakeup_rate);
        self
    }

    /// Enables the external clock selection.
    pub fn ext_clk(mut self, ext_clk: ExtClk) -> Self {
        self.config.ext_clk = Some(ext_clk);
        self
    }

    /// Enables the external sync selection.
    pub fn ext_sync(mut self, ext_sync: ExtSync) -> Self {
        self.config.ext_sync = Some(ext_sync);
        self
    }

    /// Finalizes the builder and returns the [`Config`].
    pub fn build(self) -> Config {
        self.config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            odr: OutputDataRate::Od6400Hz,
            bandwidth: Bandwidth::Bw1600Hz,
            wakeup_rate: None,
            ext_clk: None,
            ext_sync: None,
        }
    }
}

/// Validation errors generated while verifying a [`Config`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// Requested bandwidth violates Nyquist sampling limits for the chosen ODR.
    NyquistViolation,
}
