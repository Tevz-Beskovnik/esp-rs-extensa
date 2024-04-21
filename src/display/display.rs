use std::vec;

use esp_idf_svc::hal::gpio::{AnyIOPin, OutputPin};
use esp_idf_svc::hal::interrupt::IntrFlags;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::spi::config::DriverConfig;
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
    pub buffer: Vec<Vec<u8>>,
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
            .baudrate(freq)
            .bit_order(BitOrder::LsbFirst)
            .queue_size(1);

        let driver_config: DriverConfig = DriverConfig {
            dma: Dma::Disabled,
            intr_flags: IntrFlags::Level1.into(),
        };

        let driver = SpiDriver::new(spi, sclk, sdo, Option::<AnyIOPin>::None, &driver_config)?;

        let device_driver = SpiDeviceDriver::new(driver, Some(cs), &config)?;

        let screen_buffer: Vec<Vec<u8>> = vec![vec![0xFF; (width / 8).into()]; height.into()];

        Ok(Self {
            buffer: screen_buffer,
            width: width,
            height: height,
            vcom: 0x00,
            device: device_driver,
            bytes_per_line: (width / 8) as u8,
        })
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
        self.buffer.fill(vec![0xFF, self.bytes_per_line]);
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let command: u8 = self.vcom | SHARPMEM_CMD_WRITE_LINE;
        let mut commands: Vec<u8> = vec![command];
        let mut num: u8 = 0;

        while (num as u16) < self.height {
            //log::info!("number: {}, h: {}", num, self.height);
            commands.push(num + 1);
            commands.append(&mut self.buffer[num as usize]);

            num += 1;
        }

        commands.push(0x00);

        self.toggle_vcom();
        self.device.write(&commands).map_err(anyhow::Error::from)
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, value: bool) -> anyhow::Result<()> {
        if x >= self.width || y >= self.height {
            return Err(anyhow::anyhow!("Dimensions out of bounds."));
        }

        let left: u8 = (x % 8) as u8;
        let whole: u16 = x - left as u16;

        if value {
            let value: u8 = SET[left as usize];
            log::info!("whole: {}, size {}", whole, self.buffer[y as usize].len());
            self.buffer[y as usize][whole as usize] |= value;
        } else {
            let value: u8 = CLR[left as usize];

            self.buffer[y as usize][whole as usize] &= value;
        }

        Ok(())
    }
}
