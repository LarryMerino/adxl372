use embedded_hal::delay::DelayNs;

use crate::config::Config;
use crate::error::{DriverResult, Error};
use crate::interface::RegisterAccess;
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
        let timing = self.read_register::<TimingRegister>()?;
        let measure = self.read_register::<MeasureRegister>()?;
        let power_control = self.read_register::<PowerCTLRegister>()?;

        let config = Config {
            odr: timing.odr,
            wakeup_rate: timing.wakeup_rate,
            clock_source: timing.clock_source,
            sync_mode: timing.sync_mode,
            overrange_detection: measure.overrange_detection,
            auto_sleep: measure.auto_sleep,
            activity_processing: measure.activity_processing,
            noise_mode: measure.noise_mode,
            bandwidth: measure.bandwidth,
            i2c_speed_mode: power_control.i2c_speed_mode,
            instant_on_threshold: power_control.instant_on_threshold,
            filter_settling_time: power_control.filter_settling_time,
            detection_low_pass_filter: power_control.detection_low_pass_filter,
            high_pass_filter: power_control.high_pass_filter,
            power_mode: power_control.power_mode,
        };

        config
            .validate()
            .map_err(Error::<IO::Error>::InvalidConfig)?;

        Ok(config)
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
