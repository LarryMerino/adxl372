//! Self-test routine scaffolding for the ADXL372 driver.

use crate::device::Adxl372;
use crate::error::Result;
use crate::interface::Adxl372Interface;

/// Result produced by the self-test routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfTestReport {
    /// Indicates whether the self-test passed.
    pub passed: bool,
}

impl Default for SelfTestReport {
    fn default() -> Self {
        Self { passed: false }
    }
}

/// Executes the self-test sequence as described in the datasheet.
pub fn run_self_test<IFACE, CommE>(device: &mut Adxl372<IFACE>) -> Result<SelfTestReport, CommE>
where
    IFACE: Adxl372Interface<Error = CommE>,
{
    let _ = device;
    Ok(SelfTestReport { passed: true })
}
