use super::{Print, Vect2D};

pub struct Printer<T> {
    cursor_position: Vect2D,
    color: T,
}

impl<T: Clone> Printer<T> {
    pub fn set_position(&mut self, cursor_position: Vect2D) {
        self.cursor_position = cursor_position;
    }

    pub fn set_color(&mut self, color: T) {
        self.color = color;
    }

    pub fn print<U>(&mut self, printable_interface: &mut U, text: &str) -> anyhow::Result<()>
    where
        U: Print<T>,
    {
        for chr in text.as_bytes().into_iter() {
            printable_interface.put_char(
                &self.cursor_position,
                *chr as char,
                self.color.clone(),
            )?;

            self.cursor_position.x += 6;
        }

        Ok(())
    }
}
