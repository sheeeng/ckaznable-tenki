use ratatui::{layout::Rect, buffer::Buffer};

use crate::app::{DropSpeed, DropColumn, DropCell};

pub mod timer;
pub mod rain;
pub mod snow;

pub trait BackgroundWidget {
    fn buf(&self) -> &Vec<DropColumn>;
    fn get_drop_char(d: DropSpeed) -> char;

    fn get_render_char(cell: &DropCell) -> char {
        Self::get_drop_char(
            if cell.contains(&DropSpeed::Slow) {
                DropSpeed::Slow
            } else if !cell.is_empty() {
                *cell.first().unwrap()
            } else {
                DropSpeed::None
            })
    }

    fn render_background(&self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            let Some(column) = self.buf().get(x as usize) else { continue; };
            let column = column.borrow();

            for y in area.top()..area.bottom() {
                let Some(cell) = column.get(y as usize) else { continue; };
                buf.get_mut(x, y).set_char(Self::get_render_char(cell));
            }
        }
    }
}
