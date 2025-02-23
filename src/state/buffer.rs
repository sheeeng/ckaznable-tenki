use std::cell::RefCell;
use std::rc::Rc;

use ratatui::layout::Rect;
use tinyvec::ArrayVec;

use super::Column;
use super::CellType;

pub struct RenderBuffer {
    pub buf: Vec<Column>,
    pub line: Vec<CellType>,
}

impl RenderBuffer {
    pub fn new(size: Rect) -> Self {
        let mut buf = Vec::with_capacity(size.width as usize);
        for _ in 0..size.width {
            let mut column = Vec::with_capacity(size.height as usize);
            for _ in 0..size.height {
                column.push(ArrayVec::<[CellType; 3]>::default());
            }

            buf.push(Rc::new(RefCell::new(column)));
        }

        Self {
            line: Vec::with_capacity(buf.len()),
            buf,
        }
    }
}

