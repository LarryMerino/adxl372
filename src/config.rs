use crate::error::ConfigError;
use crate::params::{Bandwidth, OutputDataRate, PowerMode};

pub struct Config {
    pub odr: OutputDataRate,
    pub bandwidth: Bandwidth,
    pub power_mode: PowerMode,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            odr: OutputDataRate::Hz400, 
            bandwidth: Bandwidth::Hz200,
            power_mode: PowerMode::Standby, 
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.bandwidth.max_hz() * 2 > self.odr.hz() {
            return Err(ConfigError::NyquistViolation);
        }
 
        Ok(())
    }
} 

pub struct ConfigBuilder {
    config: Config
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self { config: Config::default() }
    }

    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.config.odr = odr;
        self
    }

    pub fn bandwidth(mut self, bandwidth: Bandwidth) -> Self {
        self.config.bandwidth = bandwidth;
        self
    }

    pub fn power_mode(mut self, mode: PowerMode) -> Self {
        self.config.power_mode = mode;
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}

