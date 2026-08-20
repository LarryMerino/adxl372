//! Register access abstraction for the ADXL372 driver.
//!
//! This module defines the interface used by the high-level driver to access
//! the ADXL372 register map independently of the underlying communication
//! protocol.
//!
//! Concrete implementations, such as [`spi::SpiInterface`], are responsible
//! for translating these register operations into protocol-specific
//! transactions.

pub mod spi;

/// Provides access to the ADXL372 register map.
///
/// This trait defines the set of register operations required by the
/// high-level driver without exposing details of the underlying transport.
///
/// Implementations are responsible for handling protocol-specific framing
/// and communication while presenting register-oriented access to the driver.
pub trait RegisterAccess {
    /// Error returned by the underlying communication interface.
    type Error;

    /// Writes a value to a single register.
    fn write_register(&mut self, register: u8, value: u8) -> core::result::Result<(), Self::Error>;

    /// Reads the value of a single register.
    fn read_register(&mut self, register: u8) -> core::result::Result<u8, Self::Error>;

    /// Reads consecutive register values into the provided buffer.
    fn read_many(&mut self, register: u8, buf: &mut [u8]) -> core::result::Result<(), Self::Error>;

    /// Writes consecutive register values from the provided buffer.
    fn write_many(&mut self, register: u8, data: &[u8]) -> core::result::Result<(), Self::Error>;
}
