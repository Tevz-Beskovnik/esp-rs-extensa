pub struct Vect2D {
    pub x: u16,
    pub y: u16,
}

pub trait Draw {
    fn set_pixel(c: Vect2D, color: bool) -> anyhow::Result<()>;

    fn draw_line(c1: Vect2D, c2: Vect2D, color: bool) -> anyhow::Result<()>;

    fn draw_hline(c: Vect2D, len: u16, color: bool) -> anyhow::Result<()>;

    fn draw_vline(c: Vect2D, hight: u16, color: bool) -> anyhow::Result<()>;

    fn draw_buffer(c: Vect2D, buffer: &[u8]) -> anyhow::Result<()>;
}

pub trait Print {
    fn put_char(c: Vect2D, chr: char) -> anyhow::Result<()>;
}
