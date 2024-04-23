use std::mem::swap;

use esp_idf_svc::sys::abs;

use crate::display::Display;

const SET: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const CLR: [u8; 8] = [!1, !2, !4, !8, !16, !32, !64, !128];
const SHO: [u8; 7] = [
    0b01111111, 0b00111111, 0b00011111, 0b00001111, 0b00000111, 0b00000011, 0b00000001,
];
const CAP: [u8; 7] = [
    !0b01111111,
    !0b00111111,
    !0b00011111,
    !0b00001111,
    !0b00000111,
    !0b00000011,
    !0b00000001,
];

pub struct Vect2D {
    pub x: u16,
    pub y: u16,
}

pub trait SetPixel {
    fn set_pixel(&mut self, c: Vect2D, color: bool) -> anyhow::Result<()>;
}

pub trait Print {
    fn put_char(&mut self, c: Vect2D, chr: char) -> anyhow::Result<()>;
}

pub trait Draw: SetPixel {
    fn draw_line(&mut self, c1: Vect2D, c2: Vect2D, color: bool) -> anyhow::Result<()>;

    fn draw_hline(&mut self, c: Vect2D, len: u16, color: bool) -> anyhow::Result<()>;

    fn draw_vline(&mut self, c: Vect2D, hight: u16, color: bool) -> anyhow::Result<()>;
}

pub struct Graphics<'a> {
    pub display: &'a (dyn Display + 'a),
    pub buffer: Vec<Vec<u8>>,
    pub width: u16,
    pub height: u16,
    bytes_pre_row: u16,
}

impl<'a> Graphics<'a> {
    pub fn new(display: &'a dyn Display, width: u16, height: u16) -> Self {
        Graphics {
            display: display,
            buffer: vec![vec![0xFF; (width / 8) as usize]; height as usize],
            width: width,
            height: height,
            bytes_pre_row: width / 8,
        }
    }

    pub fn draw(&self) -> anyhow::Result<()> {
        self.display.refresh(&self.buffer)
    }
}

impl SetPixel for Graphics<'_> {
    fn set_pixel(&mut self, c: Vect2D, color: bool) -> anyhow::Result<()> {
        let left: u8 = (c.x % 8) as u8;
        let whole: u16 = (c.x - left as u16) / 8;

        if color {
            self.buffer[c.y as usize][whole as usize] |= SET[left as usize];
        } else {
            self.buffer[c.y as usize][whole as usize] &= CLR[left as usize];
        }

        Ok(())
    }
}

impl Draw for Graphics<'_> {
    fn draw_line(&mut self, mut c1: Vect2D, mut c2: Vect2D, color: bool) -> anyhow::Result<()> {
        if c1.x >= self.width || c1.y >= self.height || c2.x >= self.width || c2.y >= self.height {
            return Err(anyhow::anyhow!("Values out of bounds"));
        }

        let steep = unsafe { abs((c2.y - c1.y).into()) > abs((c2.x - c1.x).into()) };

        if steep {
            swap(&mut c1.x, &mut c1.y);
            swap(&mut c2.x, &mut c2.y);
        }

        if c1.x > c2.x {
            swap(&mut c1, &mut c2);
        }

        let dx = c2.x - c1.x;
        let dy = unsafe { abs((c2.y - c1.y).into()) } as u16;
        let mut err: i32 = (dx / 2).into();

        while c1.x < c2.x {
            c1.x += 1;

            if steep {
                self.set_pixel(Vect2D { x: c1.y, y: c1.x }, color);
            } else {
                self.set_pixel(Vect2D { x: c1.x, y: c1.y }, color);
            }

            err -= dy as i32;

            if err < 0 {
                if c1.y < c2.y {
                    c1.y += 1;
                } else {
                    c1.y -= 1;
                }
                err += dx as i32;
            }
        }

        Ok(())
    }

    fn draw_hline(&mut self, c: Vect2D, len: u16, color: bool) -> anyhow::Result<()> {
        let left_overlap = c.x % 8;
        let right_overlap = (c.x + len) % 8;

        if (8 - left_overlap) > len {
            if color {
                self.buffer[c.y as usize][((c.x - left_overlap) / 8) as usize] |=
                    CAP[(7 - left_overlap) as usize] & SHO[(7 - left_overlap - len) as usize];
            } else {
                self.buffer[c.y as usize][((c.x - left_overlap) / 8) as usize] &=
                    SHO[(7 - left_overlap) as usize] | CAP[(7 - left_overlap - len) as usize];
            }

            return Ok(());
        }

        Ok(())
    }

    fn draw_vline(&mut self, c: Vect2D, hight: u16, color: bool) -> anyhow::Result<()> {}
}
