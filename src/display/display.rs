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

const SHARPMEM_CMD_WRITE_LINE: u8 = 0b00000001;
const SHARPMEM_CMD_VCOM: u8 = 0b00000010;
const SHARPMEM_CMD_CLEAR_SCREEN: u8 = 0b00000100;

const SET: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const CLR: [u8; 8] = [!1, !2, !4, !8, !16, !32, !64, !128];

pub struct Display<'a> {
    pub buffer: Vec<u8>,
    pub width: u16,
    pub height: u16,
    vcom: u8,
    device: SpiDeviceDriver<'a, SpiDriver<'a>>,
    bytes_per_line: u8,
}

impl<'b> Display<'b> {
    pub fn new(
        freq: Hertz,
        sclk: impl Peripheral<P = impl OutputPin> + 'b,
        sdo: impl Peripheral<P = impl OutputPin> + 'b,
        cs: impl Peripheral<P = impl OutputPin> + 'b,
        spi: impl Peripheral<P = impl SpiAnyPins> + 'b,
        width: u16,
        height: u16,
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

        let mut screen_buffer: Vec<u8> = vec![0xFF; 2 + ((width / 8 + 2) * height) as usize];

        for i in 0..height as u8 {
            screen_buffer[1 + ((width / 8 + 2) * (i as u16)) as usize] = i + 1;
            screen_buffer[(1 + (width / 8 + 1) + ((width / 8 + 2) * (i as u16))) as usize] = 0x00;
        }

        screen_buffer[(1 + (width / 8 + 2) * height) as usize] = 0x00;

        Ok(Self {
            buffer: screen_buffer,
            width: width,
            height: height,
            vcom: 0x00,
            device: device_driver,
            bytes_per_line: (width / 8) as u8,
        })
    }

    fn calc_offset(&self, x: u16, y: u16) -> usize {
        let left = x - (x % 8);
        (1 + y * (2 + self.bytes_per_line as u16) + (1 + left / 8)) as usize
    }

    fn toggle_vcom(&mut self) {
        self.vcom = if self.vcom != 0x00 {
            0x00
        } else {
            SHARPMEM_CMD_VCOM
        };
    }

    pub fn clear_display(&mut self) -> anyhow::Result<()> {
        let command: [u8; 2] = [self.vcom | SHARPMEM_CMD_CLEAR_SCREEN, 0];
        self.toggle_vcom();
        self.device.write(&command).map_err(anyhow::Error::from)
    }

    pub fn clear_buffer(&mut self) {
        let mut screen_buffer: Vec<u8> =
            vec![0xFF; 2 + ((self.width / 8 + 2) * self.height) as usize];

        for i in 0..self.height as u8 {
            screen_buffer[1 + ((self.width / 8 + 2) * (i as u16)) as usize] = i + 1;
            screen_buffer
                [(1 + (self.width / 8 + 1) + ((self.width / 8 + 2) * (i as u16))) as usize] = 0x00;
        }

        screen_buffer[(1 + (self.width / 8 + 2) * self.height) as usize] = 0x00;

        self.buffer = screen_buffer;
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let command: u8 = self.vcom | SHARPMEM_CMD_WRITE_LINE;
        self.buffer[0] = command;

        self.toggle_vcom();
        self.device.write(&self.buffer).map_err(anyhow::Error::from)
    }

    pub fn refresh_line(&mut self, line_num: u8) -> anyhow::Result<()> {
        if line_num as u16 > self.height {
            return Err(anyhow::anyhow!(
                "Line number biger then height! (ln: {}, h: {})",
                line_num,
                self.height
            ));
        }

        let command: u8 = self.vcom | SHARPMEM_CMD_WRITE_LINE;
        let mut row = vec![0x00; (self.bytes_per_line + 4) as usize];
        row[0] = command;
        row[1] = line_num;

        for i in 0..self.bytes_per_line {
            row[(i + 2) as usize] =
                self.buffer[((2 + (2 + self.bytes_per_line) * line_num) + i) as usize];
        }

        self.toggle_vcom();
        self.device.write(&row).map_err(anyhow::Error::from)
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, value: bool) -> anyhow::Result<()> {
        if x >= self.width || y >= self.height {
            return Err(anyhow::anyhow!("Dimensions out of bounds."));
        }

        let left: u8 = (x % 8) as u8;
        let offset = self.calc_offset(x, y);

        if value {
            let value: u8 = SET[left as usize];

            self.buffer[offset] |= value;
        } else {
            let value: u8 = CLR[left as usize];

            self.buffer[offset] &= value;
        }

        Ok(())
    }
}
