//! High-level ADXL372 device driver implementation.

use crate::config::Config;
use crate::error::{Error, Result};
use crate::fifo::{FifoSettings, Sample};
use crate::interface::spi::SpiInterface;
use crate::interface::Adxl372Interface;
use crate::params::{
    Bandwidth,
    ExtClk,
    ExtSync,
    FifoFormat,
    FifoMode,
    InstantOnThreshold,
    LinkLoopMode,
    LowNoise,
    OutputDataRate,
    PowerMode,
    SettleFilter,
    WakeUpRate,
};
use crate::registers::{Status, Status2};
use crate::self_test::{run_self_test, SelfTestReport};
use embedded_hal::spi::SpiDevice;

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

    /// Provides mutable access to the underlying interface.
    pub fn interface_mut(&mut self) -> &mut IFACE {
        &mut self.interface
    }
}

impl<SPI> Adxl372<SpiInterface<SPI>>
where
    SPI: SpiDevice,
{
    /// Convenience constructor for SPI transports.
    pub fn new_spi(spi: SPI, config: Config) -> Self {
        Self::new(SpiInterface::new(spi), config)
    }

    /// Releases the driver, returning the SPI device and configuration.
    pub fn release_spi(self) -> (SPI, Config) {
        let (iface, config) = self.release();
        (iface.release(), config)
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

    /// Issues a soft reset sequence.
    pub fn reset(&mut self) -> Result<(), CommE> {
        Err(Error::NotReady)
    }

    /// Reads the identification registers and returns raw bytes.
    pub fn device_id(&mut self) -> Result<[u8; 4], CommE> {
        Err(Error::NotReady)
    }

    /// Returns the raw status register bitfields.
    pub fn read_status(&mut self) -> Result<(Status, Status2), CommE> {
        Err(Error::NotReady)
    }

    /// Snapshot of FIFO configuration registers.
    pub fn fifo_settings(&mut self) -> Result<FifoSettings, CommE> {
        Err(Error::NotReady)
    }

    /// Places the sensor in the requested power mode.
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), CommE> {
        let _ = mode;
        Err(Error::NotReady)
    }

    /// Updates timing-related register fields.
    pub fn configure_timing(
        &mut self,
        odr: Option<OutputDataRate>,
        wakeup_rate: Option<WakeUpRate>,
        ext_clk: Option<ExtClk>,
        ext_sync: Option<ExtSync>,
    ) -> Result<(), CommE> {
        let _ = odr;
        let _ = wakeup_rate;
        let _ = ext_clk;
        let _ = ext_sync;
        Err(Error::NotReady)
    }

    /// Updates FIFO format, mode, or watermark.
    pub fn configure_fifo(
        &mut self,
        format: Option<FifoFormat>,
        mode: Option<FifoMode>,
        watermark: Option<u16>,
    ) -> Result<(), CommE> {
        let _ = format;
        let _ = mode;
        let _ = watermark;
        Err(Error::NotReady)
    }

    /// Adjusts measurement bandwidth, noise, and link/loop settings.
    pub fn configure_measurement(
        &mut self,
        linkloop: Option<LinkLoopMode>,
        low_noise: Option<LowNoise>,
        bandwidth: Option<Bandwidth>,
    ) -> Result<(), CommE> {
        let _ = linkloop;
        let _ = low_noise;
        let _ = bandwidth;
        Err(Error::NotReady)
    }

    /// Sets the instant-on threshold selection.
    pub fn set_instant_on_threshold(
        &mut self,
        threshold: InstantOnThreshold,
    ) -> Result<(), CommE> {
        let _ = threshold;
        Err(Error::NotReady)
    }

    /// Configures the filter settle timing.
    pub fn set_filter_settle(&mut self, settle: SettleFilter) -> Result<(), CommE> {
        let _ = settle;
        Err(Error::NotReady)
    }

    /// Reads a raw acceleration triplet.
    pub fn read_xyz_raw(&mut self) -> Result<[i16; 3], CommE> {
        Err(Error::NotReady)
    }

    /// Returns acceleration scaled in milli-g.
    pub fn read_xyz_mg(&mut self) -> Result<[i32; 3], CommE> {
        Err(Error::NotReady)
    }

    /// Returns the number of FIFO samples currently buffered.
    pub fn read_fifo_level(&mut self) -> Result<u16, CommE> {
        Err(Error::NotReady)
    }

    /// Reads raw FIFO bytes into the provided buffer.
    pub fn read_fifo_raw(&mut self, buf: &mut [u8]) -> Result<usize, CommE> {
        let _ = buf;
        Err(Error::NotReady)
    }

    /// Decodes FIFO samples into the caller-provided slice.
    pub fn read_fifo_samples(&mut self, samples: &mut [Sample]) -> Result<usize, CommE> {
        let _ = samples;
        Err(Error::NotReady)
    }

    /// Drains the FIFO without returning its contents.
    pub fn flush_fifo(&mut self) -> Result<(), CommE> {
        Err(Error::NotReady)
    }

    /// Executes the datasheet self-test routine.
    pub fn run_self_test(&mut self) -> Result<SelfTestReport, CommE> {
        run_self_test(self)
    }
}
