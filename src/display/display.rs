pub trait Display {
    fn clear_display(&mut self) -> anyhow::Result<()>;

    fn refresh(&mut self, buffer: &Vec<Vec<u8>>) -> anyhow::Result<()>;

    fn refresh_line(&mut self, line_num: u8, buffer: &[u8]) -> anyhow::Result<()>;
}
