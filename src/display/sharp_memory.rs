use std::vec;

use esp_idf_svc::hal::gpio::{AnyIOPin, OutputPin};
use esp_idf_svc::hal::interrupt::IntrFlags;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::spi::config::{DriverConfig, MODE_0};
use esp_idf_svc::hal::spi::SpiAnyPins;
use esp_idf_svc::hal::spi::{
    config::{BitOrder, Config},
    Dma, SpiDeviceDriver, SpiDriver,
};
use esp_idf_svc::hal::units::Hertz;

use crate::display::Display;

const SHARPMEM_CMD_WRITE_LINE: u8 = 0b00000001;
const SHARPMEM_CMD_VCOM: u8 = 0b00000010;
const SHARPMEM_CMD_CLEAR_SCREEN: u8 = 0b00000100;

pub struct SharpMemoryDisplay<'a> {
    vcom: u8,
    device: SpiDeviceDriver<'a, SpiDriver<'a>>,
}

impl<'b> SharpMemoryDisplay<'b> {
    pub fn new(
        freq: Hertz,
        sclk: impl Peripheral<P = impl OutputPin> + 'b,
        sdo: impl Peripheral<P = impl OutputPin> + 'b,
        cs: impl Peripheral<P = impl OutputPin> + 'b,
        spi: impl Peripheral<P = impl SpiAnyPins> + 'b,
    ) -> anyhow::Result<Self> {
        let config = Config::new()
            .data_mode(MODE_0)
            .baudrate(freq)
            .bit_order(BitOrder::LsbFirst)
            .cs_active_high()
            .queue_size(4);

        let driver_config: DriverConfig = DriverConfig {
            dma: Dma::Disabled,
            intr_flags: IntrFlags::Level1.into(),
        };

        let driver = SpiDriver::new(spi, sclk, sdo, Option::<AnyIOPin>::None, &driver_config)?;

        let device_driver = SpiDeviceDriver::new(driver, Some(cs), &config)?;

        Ok(Self {
            vcom: 0x00,
            device: device_driver,
        })
    }

    fn toggle_vcom(&mut self) {
        self.vcom = if self.vcom != 0x00 {
            0x00
        } else {
            SHARPMEM_CMD_VCOM
        };
    }
}

impl Display for SharpMemoryDisplay<'_> {
    fn clear_display(&mut self) -> anyhow::Result<()> {
        let command: [u8; 2] = [self.vcom | SHARPMEM_CMD_CLEAR_SCREEN, 0];
        self.toggle_vcom();
        self.device.write(&command).map_err(anyhow::Error::from)
    }

    fn refresh(&mut self, buffer: &Vec<Vec<u8>>) -> anyhow::Result<()> {
        let command: u8 = self.vcom | SHARPMEM_CMD_WRITE_LINE;
        let mut commands = buffer.iter().fold(vec![command], |mut acc, el| {
            acc.push(((acc.len() - 1) / (el.len() + 2) + 1) as u8); // calculate line number
            acc.extend(*el);
            acc.push(0x00);
            acc
        });

        commands.push(0x00);

        self.toggle_vcom();
        self.device.write(&commands).map_err(anyhow::Error::from)
    }

    fn refresh_line(&mut self, line_num: u8, buffer: &[u8]) -> anyhow::Result<()> {
        let command: u8 = self.vcom | SHARPMEM_CMD_WRITE_LINE;
        let mut commands: Vec<u8> = vec![command, line_num + 1];

        commands.extend(buffer);
        commands.push(0x00);
        commands.push(0x00);

        self.toggle_vcom();
        self.device.write(&commands).map_err(anyhow::Error::from)
    }
}
