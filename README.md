# ADXL372

`no_std` driver crate for the Analog Devices ADXL372 high-g 3-axis MEMS accelerometer, built on `embedded-hal` for portable use across microcontrollers.

This crate provides a safe, typed interface for working with the ADXL372 on embedded targets. It is built on [`embedded-hal`](https://docs.rs/embedded-hal/) to stay portable across microcontroller platforms  

The core driver follows the datasheet's register and timing requirements and keeps memory usage explicit by avoiding heap allocation

## Features

Optional Cargo features:

- `defmt`: enable `defmt` logging for internal debug traces

## Examples

Examples for supported microcontroller targets are available in the
`examples/` folder. The current set focuses on Espressif boards, and support
for additional MCU families can be added over time

- [ESP32-C3 (esp-hal) basic SPI read](./examples/espressif/basic/)
  - Initializes ADXL372 over SPI and reads raw X/Y/Z samples in a loop

## Usage

Import the relevant HAL crate for your platform. For this example I'm using esp-hal on ESP32C3

```rust
#![no_std]
#![no_main]

use adxl372::device::Adxl372;
use adxl372::config::Config;
use adxl372::interface::spi::SpiInterface;
use adxl372::params::{Bandwidth, OutputDataRate, PowerMode};

use embedded_hal_bus::spi::ExclusiveDevice;

...
// HAL Boilerplate
...

#[main]
fn main() -> ! {

    let config = hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = hal::init(config);

    let sclk = peripherals.GPIO6;
    let miso = peripherals.GPIO2;
    let mosi = peripherals.GPIO7;
    let cs = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());
    let spi_delay = hal::delay::Delay::new();

    let spi = hal::spi::master::Spi::new(
        peripherals.SPI2, 
        hal::spi::master::Config::default()
            .with_frequency(Rate::from_khz(400))
            .with_mode(Mode::_0),
    )
    .expect("SPI init")
    .with_sck(sclk)
    .with_miso(miso)
    .with_mosi(mosi);

    let spi_device = ExclusiveDevice::new(spi, cs, spi_delay).unwrap();

    let iface = SpiInterface::new(spi_device);
    let mut accel_3_axis = Adxl372::new(iface);

    let config = Config::new()
        .odr(OutputDataRate::Hz6400)
        .bandwidth(Bandwidth::Hz1600)
        .power_mode(PowerMode::Measurement)
        .build();

    let mut accel_delay = hal::delay::Delay::new();
    accel_3_axis.init(&mut accel_delay, config).unwrap();

    loop {
    	let [x, y, z] = accel.read_xyz_raw().unwrap();
    	let _ = (x, y, z);
    	delay.delay_ms(500);
    }
}
```

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions