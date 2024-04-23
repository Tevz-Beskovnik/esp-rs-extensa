use anyhow::Result;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, peripherals::Peripherals};
use esp_rs_extensa::display::SharpMemoryDisplay;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let mut led = PinDriver::output(peripherals.pins.gpio18)?;

    let mut display = SharpMemoryDisplay::new(
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
        display.set_pixel(0, 10, false)?;
        display.set_pixel(0, 11, false)?;
        display.set_pixel(2, 10, false)?;
        display.set_pixel(2, 11, false)?;
        display.set_pixel(4, 10, false)?;
        display.set_pixel(4, 11, false)?;
        display.set_pixel(6, 10, false)?;
        display.set_pixel(6, 11, false)?;
        display.set_pixel(8, 10, false)?;
        display.set_pixel(8, 11, false)?;
        display.set_pixel(10, 10, false)?;
        display.set_pixel(10, 11, false)?;
        display.set_pixel(12, 10, false)?;
        display.set_pixel(12, 11, false)?;
        display.set_pixel(14, 10, false)?;
        display.set_pixel(14, 11, false)?;
        display.set_pixel(16, 10, false)?;
        display.set_pixel(16, 11, false)?;
        display.refresh()?;
        led.set_high()?;
        log::info!("Set High!");
        delay.delay_us(1000000);
        /*display.set_pixel(10, 10, false)?;
        display.set_pixel(10, 11, false)?;
        display.set_pixel(11, 10, false)?;
        display.set_pixel(11, 11, false)?;
        display.refresh()?;*/
        display.clear_display()?;
        led.set_low()?;
        log::info!("Set Low!");
        delay.delay_us(1000000);
    }
}
