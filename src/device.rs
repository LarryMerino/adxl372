//! High-level ADXL372 device driver implementation.

use crate::config::Config;
use crate::error::{Error, Result};
use crate::fifo::{FifoSettings, Sample};
use crate::interface::spi::SpiInterface;
use crate::interface::Adxl372Interface;
use crate::params::{
    AutoSleep,
    Bandwidth,
    ExtClk,
    ExtSync,
    FifoFormat,
    FifoMode,
    HpfDisable,
    InstantOnThreshold,
    I2cHsmEn,
    LinkLoopMode,
    LowNoise,
    LpfDisable,
    OutputDataRate,
    PowerMode,
    SettleFilter,
    UserOrDisable,
    WakeUpRate,
};
use crate::registers::{
    Measure,
    PowerControl,
    Status,
    Status2,
    Timing,
    EXPECTED_DEVID_AD,
    EXPECTED_DEVID_MST,
    EXPECTED_PART_ID,
    REG_DEVID_AD,
    REG_MEASURE,
    REG_POWER_CTL,
    REG_RESET,
    REG_STATUS,
    REG_TIMING,
    REG_XDATA_H,
    RESET_COMMAND,
};
use crate::self_test::{run_self_test, SelfTestReport};
use embedded_hal::spi::SpiDevice;
use embedded_hal::delay::DelayNs;

// ADXL372 datasheet power-up to standby delay (milliseconds).
const POWER_UP_TO_STANDBY_DELAY_MS: u32 = 5;
// Number of consecutive bytes spanning X, Y, Z axis samples.
const RAW_AXIS_BYTES: usize = 6;

/// High-level synchronous driver for the ADXL372 accelerometer.
pub struct Adxl372<IFACE> {
    interface: IFACE,
    config: Config,
}

/// Combined view of the `STATUS` and `STATUS2` registers with explicit flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusSnapshot {
    /// STATUS[7] ERR_USER_REGS.
    pub err_user_regs: bool,
    /// STATUS[6] AWAKE.
    pub awake: bool,
    /// STATUS[5] USER_NVM_BUSY.
    pub user_nvm_busy: bool,
    /// STATUS[3] FIFO_OVR.
    pub fifo_ovr: bool,
    /// STATUS[2] FIFO_FULL.
    pub fifo_full: bool,
    /// STATUS[1] FIFO_RDY.
    pub fifo_rdy: bool,
    /// STATUS[0] DATA_RDY.
    pub data_rdy: bool,
    /// STATUS2[6] ACTIVITY2.
    pub activity2: bool,
    /// STATUS2[5] ACTIVITY.
    pub activity: bool,
    /// STATUS2[4] INACT.
    pub inact: bool,
}

impl StatusSnapshot {
    /// Builds a snapshot from the raw STATUS and STATUS2 bitfields.
    pub fn from_registers(status: Status, status2: Status2) -> Self {
        Self {
            err_user_regs: status.err_user_regs(),
            awake: status.awake(),
            user_nvm_busy: status.user_nvm_busy(),
            fifo_ovr: status.fifo_overrun(),
            fifo_full: status.fifo_full(),
            fifo_rdy: status.fifo_ready(),
            data_rdy: status.data_ready(),
            activity2: status2.activity2(),
            activity: status2.activity(),
            inact: status2.inactivity(),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for StatusSnapshot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StatusSnapshot {{\n    ERR_USER_REGS: {},\n    AWAKE: {},\n    USER_NVM_BUSY: {},\n    FIFO_OVR: {},\n    FIFO_FULL: {},\n    FIFO_RDY: {},\n    DATA_RDY: {},\n    ACTIVITY2: {},\n    ACTIVITY: {},\n    INACT: {}\n}}",
            self.err_user_regs,
            self.awake,
            self.user_nvm_busy,
            self.fifo_ovr,
            self.fifo_full,
            self.fifo_rdy,
            self.data_rdy,
            self.activity2,
            self.activity,
            self.inact
        );
    }
}

impl<IFACE> Adxl372<IFACE> {
    // ==================================================================
    // == Driver Construction & Ownership ===============================
    // ==================================================================
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
    // ==================================================================
    // == SPI Convenience Constructors ==================================
    // ==================================================================
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
    // ==================================================================
    // == Initialization & Global Configuration ==========================
    // ==================================================================
    /// Initializes the sensor using the current configuration.
    ///
    /// Enforces the datasheet power-up-to-standby delay before issuing any commands so callers
    /// do not need to provide their own wait after reset or power ramp.
    pub fn init(&mut self, delay: &mut impl DelayNs) -> Result<(), CommE> {
        self.config.validate().map_err(|_| Error::InvalidConfig)?;

        delay.delay_ms(POWER_UP_TO_STANDBY_DELAY_MS);
        self.force_power_mode(PowerMode::Standby)?;
        self.reset()?;
        self.configure(self.config)?;
        Ok(())
    }

    /// Applies a new configuration to the device.
    ///
    /// Planned helper pipeline for the forthcoming register programming:
    /// 1. `apply_timing_config()` – programs `TIMING` (ODR, wake-up rate, ext sync/clk)
    /// 2. `apply_measurement_config()` – programs `MEASURE` (bandwidth, noise, link/loop)
    /// 3. `apply_power_control_config()` – programs `POWER_CTL` fields unrelated to mode
    /// 4. `apply_fifo_config()` – programs `FIFO_CTL` and watermark registers
    /// 5. `apply_activity_config()` – programs activity/inactivity threshold windows
    /// 6. `apply_interrupt_config()` – programs interrupt/fault signaling behaviour
    ///
    /// Each helper will be wired up once its corresponding register logic is implemented.
    pub fn configure(&mut self, config: Config) -> Result<(), CommE> {
        config.validate().map_err(|_| Error::InvalidConfig)?;

        self.apply_timing_config(&config)?;
        self.apply_measurement_config(&config)?;
        self.apply_power_control_config(&config)?;

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
        self
            .interface
            .write_register(REG_RESET, RESET_COMMAND)
            .map_err(Error::from)
    }

    // ==================================================================
    // == Identification & Status =======================================
    // ==================================================================
    /// Reads the identification registers and returns raw bytes.
    pub fn device_id(&mut self) -> Result<[u8; 4], CommE> {
        Err(Error::NotReady)
    }

    /// Verifies identification registers against the expected ADXL372 constants.
    pub fn check_ids(&mut self) -> Result<u8, CommE> {
        let mut ids = [0u8; 4];
        self
            .interface
            .read_many(REG_DEVID_AD, &mut ids)
            .map_err(Error::from)?;

        if ids[0] != EXPECTED_DEVID_AD
            || ids[1] != EXPECTED_DEVID_MST
            || ids[2] != EXPECTED_PART_ID
        {
            return Err(Error::DeviceIdMismatch);
        }

        Ok(ids[3])
    }

    /// Returns a snapshot of the `STATUS` and `STATUS2` registers.
    pub fn read_status(&mut self) -> Result<StatusSnapshot, CommE> {
        let mut raw = [0u8; 2];
        self
            .interface
            .read_many(REG_STATUS, &mut raw)
            .map_err(Error::from)?;

        let status = Status::from(raw[0]);
        let status2 = Status2::from(raw[1]);

        Ok(StatusSnapshot::from_registers(status, status2))
    }

    /// Snapshot of FIFO configuration registers.
    pub fn fifo_settings(&mut self) -> Result<FifoSettings, CommE> {
        Err(Error::NotReady)
    }

    // ==================================================================
    // == Power & Measurement Configuration =============================
    // ==================================================================
    /// Updates timing-related register fields.
    pub fn configure_timing(
        &mut self,
        odr: Option<OutputDataRate>,
        wakeup_rate: Option<WakeUpRate>,
        ext_clk: Option<ExtClk>,
        ext_sync: Option<ExtSync>,
    ) -> Result<(), CommE> {
        self.update_timing_config(|timing| {
            if let Some(new_odr) = odr {
                timing.set_odr(new_odr);
            }

            if let Some(rate) = wakeup_rate {
                timing.set_wake_up_rate(rate);
            }

            if let Some(clk) = ext_clk {
                timing.set_ext_clk(clk);
            }

            if let Some(sync) = ext_sync {
                timing.set_ext_sync(sync);
            }
        })
    }
    
    /// Adjusts measurement bandwidth, noise, autosleep, and link/loop settings.
    pub fn configure_measurement(
        &mut self,
        user_or_disable: Option<UserOrDisable>,
        autosleep: Option<AutoSleep>,
        linkloop: Option<LinkLoopMode>,
        low_noise: Option<LowNoise>,
        bandwidth: Option<Bandwidth>,
    ) -> Result<(), CommE> {
        self.update_measure_config(|measure| {
            if let Some(mode) = linkloop {
                measure.set_link_loop_mode(mode);
            }

            if let Some(noise) = low_noise {
                measure.set_low_noise(noise);
            }

            if let Some(bw) = bandwidth {
                measure.set_bandwidth(bw);
            }

            if let Some(setting) = user_or_disable {
                measure.set_user_or_disable(matches!(setting, UserOrDisable::Disabled));
            }

            if let Some(sleep) = autosleep {
                measure.set_autosleep(matches!(sleep, AutoSleep::Enabled));
            }
        })
    }

    /// Updates `POWER_CTL` fields that do not require additional sequencing.
    pub fn configure_power_ctl(
        &mut self,
        hpf_disable: Option<HpfDisable>,
        lpf_disable: Option<LpfDisable>,
        instant_on_threshold: Option<InstantOnThreshold>,
        filter_settle: Option<SettleFilter>,
        i2c_hsm_en: Option<I2cHsmEn>,
    ) -> Result<(), CommE> {
        self.update_power_control(|power| {
            if let Some(setting) = hpf_disable {
                power.set_hpf_disable(matches!(setting, HpfDisable::Disabled));
            }

            if let Some(setting) = lpf_disable {
                power.set_lpf_disable(matches!(setting, LpfDisable::Disabled));
            }

            if let Some(threshold) = instant_on_threshold {
                power.set_instant_on_threshold(threshold);
            }

            if let Some(settle) = filter_settle {
                power.set_filter_settle(settle);
            }

            if let Some(setting) = i2c_hsm_en {
                power.set_i2c_high_speed_enable(matches!(setting, I2cHsmEn::Enabled));
            }
        })?;
        Ok(())
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


    /// Sets the instant-on threshold selection.
    pub fn set_instant_on_threshold(
        &mut self,
        threshold: InstantOnThreshold,
    ) -> Result<(), CommE> {
        self.update_power_control(|power| power.set_instant_on_threshold(threshold))?;
        Ok(())
    }

    /// Configures the filter settle timing.
    pub fn set_filter_settle(&mut self, settle: SettleFilter) -> Result<(), CommE> {
        self.update_power_control(|power| power.set_filter_settle(settle))?;
        Ok(())
    }

    // ==================================================================
    // == Data Acquisition & FIFO =======================================
    // ==================================================================
    #[inline]
    fn unpack_axis(msb: u8, lsb: u8) -> i16 {
        // Sensor outputs 12-bit left-justified two's complement data.
        i16::from_be_bytes([msb, lsb]) >> 4
    }

    /// Reads a raw acceleration triplet.
    pub fn read_xyz_raw(&mut self) -> Result<[i16; 3], CommE> {
        let mut raw = [0u8; RAW_AXIS_BYTES];
        self
            .interface
            .read_many(REG_XDATA_H, &mut raw)
            .map_err(Error::from)?;

        let x = Self::unpack_axis(raw[0], raw[1]);
        let y = Self::unpack_axis(raw[2], raw[3]);
        let z = Self::unpack_axis(raw[4], raw[5]);

        Ok([x, y, z])
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

    // ==================================================================
    // == Self-Test ======================================================
    // ==================================================================
    /// Executes the datasheet self-test routine.
    pub fn run_self_test(&mut self) -> Result<SelfTestReport, CommE> {
        run_self_test(self)
    }

    // ==================================================================
    // == Internal Configuration Helpers =================================
    // ==================================================================

    #[allow(dead_code)]
    fn apply_timing_config(&mut self, config: &Config) -> Result<(), CommE> {
        self.update_timing_config(|timing| {
            timing.set_odr(config.odr);
            timing.set_wake_up_rate(config.wakeup_rate);
            timing.set_ext_clk(config.ext_clk);
            timing.set_ext_sync(config.ext_sync);
        })
    }

    fn update_timing_config<F>(&mut self, mut mutate: F) -> Result<(), CommE>
    where
        F: FnMut(&mut Timing),
    {
        let current = self
            .interface
            .read_register(REG_TIMING)
            .map_err(Error::from)?;

        let mut timing = Timing::from(current);
        mutate(&mut timing);

        let new_odr = timing.odr();
        if self.config.bandwidth.max_hz() * 2 > new_odr.hz() {
            return Err(Error::InvalidConfig);
        }

        let updated = u8::from(timing);
        if updated != current {
            self
                .interface
                .write_register(REG_TIMING, updated)
                .map_err(Error::from)?;
        }

        self.config.odr = new_odr;
        self.config.wakeup_rate = timing.wake_up_rate();
        self.config.ext_clk = timing.ext_clk();
        self.config.ext_sync = timing.ext_sync();

        Ok(())
    }
    
    #[allow(dead_code)]
    fn apply_measurement_config(&mut self, config: &Config) -> Result<(), CommE> {
        self.update_measure_config(|measure| {
            measure.set_bandwidth(config.bandwidth);
            measure.set_low_noise(config.low_noise);
            measure.set_link_loop_mode(config.linkloop);
            measure.set_autosleep(matches!(config.autosleep, AutoSleep::Enabled));
            measure.set_user_or_disable(matches!(config.user_or_disable, UserOrDisable::Disabled));
        })
    }

    fn update_measure_config<F>(&mut self, mut mutate: F) -> Result<(), CommE>
    where
        F: FnMut(&mut Measure),
    {
        let current = self
            .interface
            .read_register(REG_MEASURE)
            .map_err(Error::from)?;

        let mut measure = Measure::from(current);
        mutate(&mut measure);

        let new_bandwidth = measure.bandwidth();
        if new_bandwidth.max_hz() * 2 > self.config.odr.hz() {
            return Err(Error::InvalidConfig);
        }

        let updated = u8::from(measure);
        if updated != current {
            self
                .interface
                .write_register(REG_MEASURE, updated)
                .map_err(Error::from)?;
        }

        self.config.bandwidth = new_bandwidth;
        self.config.low_noise = measure.low_noise();
        self.config.linkloop = measure.link_loop_mode();
        self.config.autosleep = if measure.autosleep() {
            AutoSleep::Enabled
        } else {
            AutoSleep::Disabled
        };
        self.config.user_or_disable = if measure.user_or_disable() {
            UserOrDisable::Disabled
        } else {
            UserOrDisable::Enabled
        };

        Ok(())
    }

    #[allow(dead_code)]
    fn apply_power_control_config(&mut self, config: &Config) -> Result<(), CommE> {
        self.update_power_control(|power| {
            power.set_mode(config.power_mode);
            power.set_hpf_disable(matches!(config.hpf_disable, HpfDisable::Disabled));
            power.set_lpf_disable(matches!(config.lpf_disable, LpfDisable::Disabled));
            power.set_instant_on_threshold(config.instant_on_threshold);
            power.set_filter_settle(config.filter_settle);
            power.set_i2c_high_speed_enable(matches!(config.i2c_hsm_en, I2cHsmEn::Enabled));
        })
    }

    fn force_power_mode(&mut self, mode: PowerMode) -> Result<(), CommE> {
        self.mutate_power_control(|power| power.set_mode(mode))?;
        Ok(())
    }

    fn mutate_power_control<F>(&mut self, mut mutate: F) -> Result<PowerControl, CommE>
    where
        F: FnMut(&mut PowerControl),
    {
        let current = self
            .interface
            .read_register(REG_POWER_CTL)
            .map_err(Error::from)?;

        let mut power = PowerControl::from(current);
        mutate(&mut power);

        let updated = u8::from(power);
        if updated != current {
            self
                .interface
                .write_register(REG_POWER_CTL, updated)
                .map_err(Error::from)?;
        }

        Ok(power)
    }

    fn update_power_control<F>(&mut self, mut mutate: F) -> Result<(), CommE>
    where
        F: FnMut(&mut PowerControl),
    {
        let power = self.mutate_power_control(|ctrl| mutate(ctrl))?;

        self.config.power_mode = power.mode();
        self.config.hpf_disable = if power.hpf_disable() {
            HpfDisable::Disabled
        } else {
            HpfDisable::Enabled
        };
        self.config.lpf_disable = if power.lpf_disable() {
            LpfDisable::Disabled
        } else {
            LpfDisable::Enabled
        };
        self.config.filter_settle = power.filter_settle();
        self.config.instant_on_threshold = power.instant_on_threshold();
        self.config.i2c_hsm_en = if power.i2c_high_speed_enable() {
            I2cHsmEn::Enabled
        } else {
            I2cHsmEn::Disabled
        };

        Ok(())
    }

    #[allow(dead_code)]
    fn apply_fifo_config(&mut self, config: &Config) -> Result<(), CommE> {
        let _ = config;
        Err(Error::NotReady)
    }

    #[allow(dead_code)]
    fn apply_activity_config(&mut self, config: &Config) -> Result<(), CommE> {
        let _ = config;
        Err(Error::NotReady)
    }

    #[allow(dead_code)]
    fn apply_interrupt_config(&mut self, config: &Config) -> Result<(), CommE> {
        let _ = config;
        Err(Error::NotReady)
    }
}
