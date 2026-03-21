#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::delay::Delay;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::gpio::{Level, Output, OutputConfig};

use panic_rtt_target as _;

use embedded_hal_bus::spi::ExclusiveDevice;

use adxl372::device::Adxl372;
use adxl372::config::Config;
use adxl372::interface::spi::SpiInterface;
use adxl372::params::{Bandwidth, OutputDataRate, PowerMode};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.1

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    

    let sclk = peripherals.GPIO6;
    let miso = peripherals.GPIO2;
    let mosi = peripherals.GPIO7;
    let cs = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());
    let spi_delay = Delay::new();

    let spi = Spi::new(
        peripherals.SPI2, 
        SpiConfig::default()
            .with_frequency(Rate::from_khz(400))
            .with_mode(Mode::_0),
    )
    .expect("SPI init")
    .with_sck(sclk)
    .with_miso(miso)
    .with_mosi(mosi);

    let spi_device = ExclusiveDevice::new(spi, cs, spi_delay).unwrap();

    let iface = SpiInterface::new(spi_device);
    let config = Config::new()
        .odr(OutputDataRate::Od6400Hz)
        .bandwidth(Bandwidth::Bw1600Hz)
        .power_mode(PowerMode::Measure)
        .build();

    let mut accel_3_axis = Adxl372::new(iface, config);

    let mut accel_delay = Delay::new();
    accel_3_axis.init(&mut accel_delay).unwrap();
    let rev_id = accel_3_axis.check_ids().unwrap();

    info!("REVID: {}", rev_id);

    loop {
        let data = accel_3_axis.read_xyz_raw().unwrap();
        info!("Read X: {}, Y: {}, Z: {}", data[0], data[1], data[2]);
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples/src/bin
}
