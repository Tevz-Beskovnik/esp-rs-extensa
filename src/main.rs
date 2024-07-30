use anyhow::Result;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{delay::Delay, peripherals::Peripherals};
use esp_rs_extensa::display::SharpMemoryDisplay;
use esp_rs_extensa::filesystem::register_spiffs_partition;
use esp_rs_extensa::graphics::{Draw, MonoGraphics, Vect2D, WHITE};

const MOUNT_POINT: &str = "/spiffs";
const PARTITION_NAME: &str = "storage";

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    register_spiffs_partition(MOUNT_POINT, PARTITION_NAME)?;

    let mut display = SharpMemoryDisplay::new(
        2.MHz().into(),
        peripherals.pins.gpio25,
        peripherals.pins.gpio26,
        peripherals.pins.gpio27,
        peripherals.spi3,
    )?;

    let mut graphics = MonoGraphics::new(&mut display, 400, 240);

    log::info!("Hello, world!");

    let delay: Delay = Default::default();

    loop {
        //graphics.draw_rectangle(Vect2D { x: 20, y: 20 }, Vect2D { x: 380, y: 220 }, BLACK)?;
        //graphics.fill_rectangle(Vect2D { x: 40, y: 40 }, Vect2D { x: 360, y: 200 }, BLACK)?;
        /*for i in (0..80 as u16).step_by(3) {
            graphics.draw_rectangle(
                Vect2D {
                    x: 20 + i,
                    y: 20 + i,
                },
                Vect2D {
                    x: 380 - i,
                    y: 220 - i,
                },
                BLACK,
            )?;
        }



        graphics.draw_rectangle(Vect2D { x: 2, y: 2 }, Vect2D { x: 3, y: 3 }, BLACK)?;*/
        /*for i in 1..20 as u16 {
            graphics.draw_hline(Vect2D { x: 0, y: i }, i, BLACK)?;
            graphics.draw_vline(Vect2D { x: 19 - i, y: 20 }, i, BLACK)?;
        }*/
        graphics.draw_texture_from_flash(Vect2D::new(0, 0), "/spiffs/land.img")?;
        graphics.draw()?;
        log::info!("draw display");
        delay.delay_us(1000000);
        /*display.set_pixel(0, 10, false)?;
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
        led.set_low()?;*/
        graphics.clear(WHITE)?;
        graphics.clear_display()?;
        log::info!("Clear display");
        delay.delay_us(1000000);
    }
}
