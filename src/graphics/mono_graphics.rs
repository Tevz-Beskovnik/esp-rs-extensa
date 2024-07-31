use std::{borrow::BorrowMut, mem::swap};

use anyhow::anyhow;
use esp_idf_svc::sys::abs;

use crate::display::Display;

use super::glcdfont::GLCD_FONT;
use super::{Draw, Print, SetPixel, Vect2D};

pub const WHITE: bool = true;
pub const BLACK: bool = false;

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

pub struct MonoGraphics<'a> {
    pub display: &'a mut (dyn Display + 'a),
    pub buffer: Vec<Vec<u8>>,
    pub width: u16,
    pub height: u16,
}

impl<'a> MonoGraphics<'a> {
    pub fn new(display: &'a mut dyn Display, width: u16, height: u16) -> Self {
        MonoGraphics {
            display: display,
            buffer: vec![vec![0xFF; (width / 8) as usize]; height as usize],
            width: width,
            height: height,
        }
    }

    pub fn clear_display(&mut self) -> anyhow::Result<()> {
        self.display.clear_display()
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.display.borrow_mut().refresh(&self.buffer)
    }
}

impl SetPixel<bool> for MonoGraphics<'_> {
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

impl Draw<bool> for MonoGraphics<'_> {
    fn clear(&mut self, color: bool) -> anyhow::Result<()> {
        let line_color = if color { 0xFF } else { 0x00 };

        for i in 0..self.buffer.len() {
            self.buffer[i].fill(line_color);
        }

        Ok(())
    }

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
                self.set_pixel(Vect2D { x: c1.y, y: c1.x }, color)?;
            } else {
                self.set_pixel(Vect2D { x: c1.x, y: c1.y }, color)?;
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
        let right_overflow = (c.x + len) % 8;

        if (8 - left_overlap) > len && left_overlap != 0 {
            if color {
                self.buffer[c.y as usize][((c.x - left_overlap) / 8) as usize] |=
                    CAP[(7 - left_overlap) as usize] & SHO[(7 - left_overlap - len) as usize];
            } else {
                self.buffer[c.y as usize][((c.x - left_overlap) / 8) as usize] &=
                    SHO[(7 - left_overlap) as usize] | CAP[(7 - left_overlap - len) as usize];
            }

            return Ok(());
        }

        if left_overlap != 0 {
            if color {
                self.buffer[c.y as usize][((c.x - left_overlap) / 8) as usize] |=
                    CAP[(7 - left_overlap) as usize];
            } else {
                self.buffer[c.y as usize][((c.x - left_overlap) / 8) as usize] &=
                    SHO[(7 - left_overlap) as usize];
            }
        }

        if right_overflow != 0 {
            if color {
                self.buffer[c.y as usize][((c.x + len - right_overflow) / 8) as usize] |=
                    SHO[(7 - right_overflow) as usize];
            } else {
                self.buffer[c.y as usize][((c.x + len - right_overflow) / 8) as usize] &=
                    CAP[(7 - right_overflow) as usize];
            }
        }

        for i in (c.x + 7) / 8..(c.x + (len - right_overflow)) / 8 {
            self.buffer[c.y as usize][i as usize] = color as u8 * 0xff;
        }

        Ok(())
    }

    fn draw_vline(&mut self, c: Vect2D, height: u16, color: bool) -> anyhow::Result<()> {
        if c.x >= self.width || c.y >= self.height || c.y + height >= self.width {
            return Err(anyhow!("Line dimensions out of bounds"));
        }

        let offset = c.x % 8;
        let coord = (c.x - offset) / 8;

        for i in 0..height {
            if color {
                self.buffer[(c.y + i) as usize][coord as usize] |= SET[offset as usize];
            } else {
                self.buffer[(c.y + i) as usize][coord as usize] &= CLR[offset as usize];
            }
        }

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        corner1: Vect2D,
        corner2: Vect2D,
        color: bool,
    ) -> anyhow::Result<()> {
        unsafe {
            self.draw_hline(
                corner1,
                abs(corner1.x as i32 - corner2.x as i32 - 1).try_into()?,
                color,
            )?;

            self.draw_hline(
                Vect2D {
                    x: corner1.x,
                    y: corner2.y,
                },
                abs(corner1.x as i32 - corner2.x as i32 - 1).try_into()?,
                color,
            )?;

            self.draw_vline(
                corner1,
                abs(corner1.y as i32 - corner2.y as i32 - 1).try_into()?,
                color,
            )?;

            self.draw_vline(
                Vect2D {
                    x: corner2.x,
                    y: corner1.y,
                },
                abs(corner1.y as i32 - corner2.y as i32 - 1).try_into()?,
                color,
            )?;
        }

        Ok(())
    }

    fn fill_rectangle(
        &mut self,
        corner1: Vect2D,
        corner2: Vect2D,
        color: bool,
    ) -> anyhow::Result<()> {
        unsafe {
            for i in 0..abs(corner1.y as i32 - corner2.y as i32).try_into()? {
                self.draw_hline(
                    Vect2D {
                        x: corner1.x,
                        y: corner1.y + i,
                    },
                    abs(corner1.x as i32 - corner2.x as i32).try_into()?,
                    color,
                )?;
            }
        }

        Ok(())
    }

    fn draw_texture(&mut self, corner: Vect2D, texture: &Vec<Vec<u8>>) -> anyhow::Result<()> {
        if texture.len() == 0 {
            return Ok(());
        }

        for i in corner.y..texture.len().clamp(0, self.height as usize) as u16 {
            for j in corner.x..texture[0].len().clamp(0, self.width as usize) as u16 {
                self.buffer[i as usize][j as usize] = texture[i as usize][j as usize];
            }
        }

        Ok(())
    }

    fn draw_texture_from_flash(&mut self, corner: Vect2D, path: &str) -> anyhow::Result<()> {
        if corner.x >= self.width || corner.y >= self.height {
            return Err(anyhow!("Corner coordinates are out of screen bounds"));
        }

        let texture = std::fs::read(path)?;

        let w = texture[0] as u16 | ((texture[1] as u16) << 8);
        let h = texture[2] as u16 | ((texture[3] as u16) << 8);
        let acutal_x = corner.x + 8 - (corner.x % 8);
        let total_h = corner.y + h;
        let total_w = acutal_x + w;

        for i in corner.y..total_h - (total_h % self.height) {
            for j in acutal_x / 8..(total_w - (total_w % self.width)) / 8 {
                self.buffer[i as usize][j as usize] =
                    texture[(i * self.width / 8 + j + 4) as usize];
            }
        }

        Ok(())
    }
}

impl Print<bool> for MonoGraphics<'_> {
    fn put_char(&mut self, c: &Vect2D, chr: char, color: bool) -> anyhow::Result<()> {
        if c.x >= self.width || c.y >= self.height {
            return Ok(());
        }

        for i in 0..5 {
            let mut line: u8 = GLCD_FONT[(chr as usize) * 5 + i];

            for j in 0..8 {
                if (line & 1) == 1 {
                    self.set_pixel(
                        Vect2D {
                            x: c.x + i as u16,
                            y: c.y + j,
                        },
                        color,
                    )?;
                } else {
                    self.set_pixel(
                        Vect2D {
                            x: c.x + i as u16,
                            y: c.y + j,
                        },
                        !color,
                    )?;
                }

                line >>= 1;
            }
        }

        self.draw_vline(Vect2D { x: c.x + 5, y: c.y }, 8, !color)?;

        Ok(())
    }
}
