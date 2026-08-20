//! SPI interface implementation built on top of `embedded-hal` `SpiDevice`.

use embedded_hal::spi::{Operation, SpiDevice};

use super::RegisterAccess;

/// SPI-based interface implementation for the ADXL372 driver.
pub struct SpiInterface<SPI> {
    spi: SPI,
}

impl<SPI> SpiInterface<SPI> {
    /// Creates a new interface from the provided SPI device abstraction.
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Builds the command byte used to address registers over SPI.
    fn command_byte(register: u8, is_read: bool) -> u8 {
        let mut command = (register & 0x7F) << 1;
        if is_read {
            command |= 0x01;
        }
        command
    }

    /// Provides mutable access to the wrapped SPI device.
    pub fn spi_mut(&mut self) -> &mut SPI {
        &mut self.spi
    }

    /// Consumes the interface and returns the owned SPI device.
    pub fn release(self) -> SPI {
        self.spi
    }
}

impl<SPI> RegisterAccess for SpiInterface<SPI>
where
    SPI: SpiDevice,
{
    type Error = SPI::Error;

    fn write_register(&mut self, register: u8, value: u8) -> core::result::Result<(), Self::Error> {
        self.write_many(register, core::slice::from_ref(&value))
    }

    fn read_register(&mut self, register: u8) -> core::result::Result<u8, Self::Error> {
        let mut value = [0u8; 1];
        self.read_many(register, &mut value)?;
        Ok(value[0])
    }

    fn read_many(&mut self, register: u8, buf: &mut [u8]) -> core::result::Result<(), Self::Error> {
        if buf.is_empty() {
            return Ok(());
        }

        let command = [Self::command_byte(register, true)];
        let mut operations = [Operation::Write(&command), Operation::Read(buf)];
        self.spi.transaction(&mut operations)
    }

    fn write_many(&mut self, register: u8, data: &[u8]) -> core::result::Result<(), Self::Error> {
        if data.is_empty() {
            return Ok(());
        }

        let command = [Self::command_byte(register, false)];
        let mut operations = [Operation::Write(&command), Operation::Write(data)];
        self.spi.transaction(&mut operations)
    }
}
