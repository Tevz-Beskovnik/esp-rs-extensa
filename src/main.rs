use anyhow::Result;
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, peripherals::Peripherals};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let mut led = PinDriver::output(peripherals.pins.gpio18)?;

    log::info!("Hello, world!");

    let delay: Delay = Default::default();

    loop {
        led.set_high()?;
        log::info!("Set High!");
        delay.delay_us(100000);
        led.set_low()?;
        log::info!("Set Low!");
        delay.delay_us(100000);
    }
}
