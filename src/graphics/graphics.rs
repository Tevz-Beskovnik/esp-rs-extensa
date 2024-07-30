#[derive(Clone, Copy)]
pub struct Vect2D {
    pub x: u16,
    pub y: u16,
}

impl Vect2D {
    pub fn new(x: u16, y: u16) -> Self {
        Vect2D { x, y }
    }
}

pub trait SetPixel<T> {
    fn set_pixel(&mut self, c: Vect2D, color: T) -> anyhow::Result<()>;
}

pub trait Print<T> {
    fn put_char(&mut self, c: &Vect2D, chr: char, color: T) -> anyhow::Result<()>;
}

pub trait Draw<T>: SetPixel<T> {
    fn clear(&mut self, color: T) -> anyhow::Result<()>;

    fn draw_line(&mut self, c1: Vect2D, c2: Vect2D, color: T) -> anyhow::Result<()>;

    fn draw_hline(&mut self, c: Vect2D, len: u16, color: T) -> anyhow::Result<()>;

    fn draw_vline(&mut self, c: Vect2D, height: u16, color: T) -> anyhow::Result<()>;

    fn draw_rectangle(&mut self, corner1: Vect2D, corner2: Vect2D, color: T) -> anyhow::Result<()>;

    fn fill_rectangle(&mut self, corner1: Vect2D, corner2: Vect2D, color: T) -> anyhow::Result<()>;

    fn draw_texture(&mut self, corner: Vect2D, texture: &Vec<Vec<u8>>) -> anyhow::Result<()>;

    fn draw_texture_from_flash(&mut self, corner: Vect2D, path: &str) -> anyhow::Result<()>;
}
