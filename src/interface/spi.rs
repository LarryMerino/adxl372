//! SPI interface implementation placeholder.

use embedded_hal::spi::SpiDevice;

use super::Adxl372Interface;

/// SPI-based interface implementation for the ADXL372 driver.
pub struct SpiInterface<SPI> {
    spi: SPI,
}

impl<SPI> SpiInterface<SPI> {
    /// Creates a new interface from the provided SPI device abstraction.
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
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

impl<SPI> Adxl372Interface for SpiInterface<SPI>
where
    SPI: SpiDevice,
{
    type Error = SPI::Error;

    fn write_register(&mut self, register: u8, value: u8) -> core::result::Result<(), Self::Error> {
        let _ = register;
        let _ = value;
        Ok(())
    }

    fn read_register(&mut self, register: u8) -> core::result::Result<u8, Self::Error> {
        let _ = register;
        Ok(0)
    }

    fn read_many(&mut self, register: u8, buf: &mut [u8]) -> core::result::Result<(), Self::Error> {
        let _ = register;
        for byte in buf.iter_mut() {
            *byte = 0;
        }
        Ok(())
    }

    fn write_many(&mut self, register: u8, data: &[u8]) -> core::result::Result<(), Self::Error> {
        let _ = register;
        let _ = data;
        Ok(())
    }
}
