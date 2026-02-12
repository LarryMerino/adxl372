//! Self-test routine implementation for the ADXL372 driver.

use embedded_hal::delay::DelayNs;

use crate::device::Adxl372;
use crate::error::{Error, Result};
use crate::interface::Adxl372Interface;
use crate::params::{PowerMode, SettleFilter};
use crate::registers::{PowerControl, REG_POWER_CTL, REG_SELF_TEST, SelfTest as SelfTestReg};

const SELF_TEST_THRESHOLD_LSB: i16 = 5;
const SELF_TEST_TIMEOUT_MS: u16 = 500;
const SELF_TEST_ACTIVATION_GUARD_MS: u16 = 2;
const SELF_TEST_SAMPLE_PERIOD_NS: u32 = 2_500_000; // 400 Hz -> 2.5 ms
const SELF_TEST_FILTER_SETTLE: SettleFilter = SettleFilter::Ms370;
const SELF_TEST_SETTLE_DELAY_MS: u32 = SELF_TEST_FILTER_SETTLE.millis() as u32;
// Datasheet self-test errata requires averaging the first and last 50 ms at the default 400 Hz ODR
// (see documents/adxl372-2.txt#L5596-L5608), yielding 20 samples per window.
const SELF_TEST_SAMPLES_PER_WINDOW: usize = 20;

/// Result produced by the self-test routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfTestReport {
    /// Indicates whether the self-test passed.
    pub passed: bool,
    /// Average Z-axis reading before excitation (LSB units).
    pub baseline_avg_z: i16,
    /// Average Z-axis reading after excitation (LSB units).
    pub stimulated_avg_z: i16,
    /// Difference between stimulated and baseline averages (LSB units).
    pub delta_z_lsb: i16,
    /// Number of samples captured within each 50 ms window.
    pub samples_per_window: u16,
    /// Reflects the hardware `USER_ST` flag captured after completion.
    pub user_flag: bool,
    /// Indicates whether the procedure timed out waiting for completion.
    pub timed_out: bool,
}

impl Default for SelfTestReport {
    fn default() -> Self {
        Self {
            passed: false,
            baseline_avg_z: 0,
            stimulated_avg_z: 0,
            delta_z_lsb: 0,
            samples_per_window: 0,
            user_flag: false,
            timed_out: false,
        }
    }
}

/// Executes the datasheet/errata self-test sequence using the sensor defaults.
///
/// The routine performs a soft reset before sampling to ensure all registers match the
/// factory defaults (ODR 400 Hz, bandwidth 200 Hz, measurement mode). It also issues a
/// final reset before returning so the caller can safely apply their own configuration.
/// Always invoke this API **before** calling [`Adxl372::init`](crate::device::Adxl372::init)
/// or any configuration helper because the concluding reset clears prior register writes.
pub fn run_self_test<IFACE, CommE>(
    device: &mut Adxl372<IFACE>,
    delay: &mut impl DelayNs,
) -> Result<SelfTestReport, CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    device.reset()?;
    configure_power_control_for_self_test(device)?;
    if SELF_TEST_SETTLE_DELAY_MS > 0 {
        delay.delay_ms(SELF_TEST_SETTLE_DELAY_MS);
    }

    let result = execute_self_test_sequence(device, delay);

    match result {
        Ok(report) => {
            device.reset()?;
            Ok(report)
        }
        Err(err) => {
            let _ = device.reset();
            Err(err)
        }
    }
}

fn execute_self_test_sequence<IFACE, CommE>(
    device: &mut Adxl372<IFACE>,
    delay: &mut impl DelayNs,
) -> Result<SelfTestReport, CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    clear_self_test_trigger(device)?;

    if SELF_TEST_ACTIVATION_GUARD_MS > 0 {
        delay.delay_ms(u32::from(SELF_TEST_ACTIVATION_GUARD_MS));
    }

    trigger_self_test(device)?;

    let window_stats = collect_self_test_windows(device, delay)?;
    clear_self_test_trigger(device)?;

    let baseline_avg = window_stats.baseline_avg;
    let stimulated_avg = window_stats.stimulated_avg;
    let delta = stimulated_avg - baseline_avg;

    let user_flag = window_stats.final_reg.user_st();
    let displacement_ok = delta.abs() >= SELF_TEST_THRESHOLD_LSB;
    let passed = !window_stats.timed_out && user_flag && displacement_ok;

    Ok(SelfTestReport {
        passed,
        baseline_avg_z: baseline_avg,
        stimulated_avg_z: stimulated_avg,
        delta_z_lsb: delta,
        samples_per_window: window_stats
            .baseline_samples
            .min(window_stats.stimulated_samples),
        user_flag,
        timed_out: window_stats.timed_out,
    })
}

fn trigger_self_test<IFACE, CommE>(device: &mut Adxl372<IFACE>) -> Result<(), CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    update_self_test_register(device, |reg| reg.set_st(true))?;
    Ok(())
}

fn clear_self_test_trigger<IFACE, CommE>(device: &mut Adxl372<IFACE>) -> Result<(), CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    update_self_test_register(device, |reg| reg.set_st(false))?;
    Ok(())
}

fn update_self_test_register<IFACE, CommE, F>(
    device: &mut Adxl372<IFACE>,
    mut mutate: F,
) -> Result<(), CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
    F: FnMut(&mut SelfTestReg),
{
    let iface = device.interface_mut();
    let current = iface.read_register(REG_SELF_TEST).map_err(Error::from)?;
    let mut reg = SelfTestReg::from(current);
    mutate(&mut reg);
    let updated = u8::from(reg);
    if updated != current {
        iface
            .write_register(REG_SELF_TEST, updated)
            .map_err(Error::from)?;
    }
    Ok(())
}

fn configure_power_control_for_self_test<IFACE, CommE>(
    device: &mut Adxl372<IFACE>,
) -> Result<(), CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    let iface = device.interface_mut();
    let current = iface.read_register(REG_POWER_CTL).map_err(Error::from)?;
    let mut reg = PowerControl::from(current);
    reg.set_lpf_disable(false);
    reg.set_mode(PowerMode::Measure);
    reg.set_filter_settle(SELF_TEST_FILTER_SETTLE);
    let updated = u8::from(reg);
    if updated != current {
        iface
            .write_register(REG_POWER_CTL, updated)
            .map_err(Error::from)?;
    }
    Ok(())
}

fn read_self_test_register<IFACE, CommE>(device: &mut Adxl372<IFACE>) -> Result<SelfTestReg, CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    let value = device
        .interface_mut()
        .read_register(REG_SELF_TEST)
        .map_err(Error::from)?;
    Ok(SelfTestReg::from(value))
}

struct SelfTestWindowStats {
    baseline_avg: i16,
    baseline_samples: u16,
    stimulated_avg: i16,
    stimulated_samples: u16,
    final_reg: SelfTestReg,
    timed_out: bool,
}

fn collect_self_test_windows<IFACE, CommE>(
    device: &mut Adxl372<IFACE>,
    delay: &mut impl DelayNs,
) -> Result<SelfTestWindowStats, CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    let target_samples = SELF_TEST_SAMPLES_PER_WINDOW as u16;
    let mut baseline_sum: i16 = 0;
    let mut baseline_count: u16 = 0;

    let mut rolling_samples = [0i16; SELF_TEST_SAMPLES_PER_WINDOW];
    let mut rolling_sum: i16 = 0;
    let mut rolling_count: u16 = 0;
    let mut rolling_index: usize = 0;

    let mut elapsed_ns: u32 = 0;
    let timeout_ns = u32::from(SELF_TEST_TIMEOUT_MS) * 1_000_000;
    let mut timed_out = false;
    let mut last_reg = read_self_test_register(device)?;

    loop {
        if elapsed_ns >= timeout_ns {
            timed_out = true;
            break;
        }

        last_reg = read_self_test_register(device)?;
        if last_reg.st_done() {
            break;
        }

        let frame = device.read_xyz_raw()?;
        let z = frame[2];

        if baseline_count < target_samples {
            baseline_sum = baseline_sum.saturating_add(z);
            baseline_count = baseline_count.saturating_add(1);
        }

        if rolling_count == target_samples {
            let removed = rolling_samples[rolling_index];
            rolling_sum = rolling_sum.saturating_add(-removed);
        } else {
            rolling_count = rolling_count.saturating_add(1);
        }
        rolling_samples[rolling_index] = z;
        rolling_sum = rolling_sum.saturating_add(z);
        rolling_index = (rolling_index + 1) % SELF_TEST_SAMPLES_PER_WINDOW;

        if SELF_TEST_SAMPLE_PERIOD_NS > 0 {
            delay.delay_ns(SELF_TEST_SAMPLE_PERIOD_NS);
            elapsed_ns = elapsed_ns.saturating_add(SELF_TEST_SAMPLE_PERIOD_NS);
        }
    }

    let baseline_divisor = baseline_count as i16;
    let baseline_avg = if baseline_divisor > 0 {
        baseline_sum / baseline_divisor
    } else {
        0
    };

    let stimulated_divisor = rolling_count as i16;
    let stimulated_avg = if stimulated_divisor > 0 {
        rolling_sum / stimulated_divisor
    } else {
        0
    };

    let timed_out = timed_out || !last_reg.st_done();

    Ok(SelfTestWindowStats {
        baseline_avg,
        baseline_samples: baseline_count,
        stimulated_avg,
        stimulated_samples: rolling_count,
        final_reg: last_reg,
        timed_out,
    })
}
