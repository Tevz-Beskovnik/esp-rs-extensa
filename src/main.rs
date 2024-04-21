use anyhow::Result;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, peripherals::Peripherals};
use esp_rs_extensa::display::display::Display;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let mut led = PinDriver::output(peripherals.pins.gpio18)?;

    let mut display = Display::new(
        2.MHz().into(),
        peripherals.pins.gpio25,
        peripherals.pins.gpio26,
        peripherals.pins.gpio27,
        peripherals.spi3,
        400,
        240,
    )?;

    log::info!("Hello, world!");

    let delay: Delay = Default::default();

    loop {
        display.set_pixel(10, 10, true)?;
        display.set_pixel(10, 11, true)?;
        display.set_pixel(11, 10, true)?;
        display.set_pixel(11, 11, true)?;
        display.refresh()?;
        led.set_high()?;
        log::info!("Set High!");
        delay.delay_us(100000);
        /*display.set_pixel(10, 10, false)?;
        display.set_pixel(10, 11, false)?;
        display.set_pixel(11, 10, false)?;
        display.set_pixel(11, 11, false)?;
        display.refresh()?;*/
        display.clear_buffer();
        led.set_low()?;
        log::info!("Set Low!");
        delay.delay_us(100000);
    }
}
