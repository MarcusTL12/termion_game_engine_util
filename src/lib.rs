use std::io::Write;

use termion::{
    color, cursor,
    event::{Event, MouseButton, MouseEvent},
};

use termion_game_engine::{col2fg_str, GameObject};

pub struct Button {
    c1: (u16, u16),
    c2: (u16, u16),
    col: Vec<u8>,
    mousepress: Option<MouseButton>,
    mouseheld: Option<MouseButton>,
    mouserelease: Option<MouseButton>,
}

impl Button {
    pub fn new<T: color::Color>(
        pos: (u16, u16),
        size: (u16, u16),
        col: T,
    ) -> Self {
        Button {
            c1: pos,
            c2: (pos.0 + size.0 - 1, pos.1 + size.1 - 1),
            col: col2fg_str(col),
            mousepress: None,
            mouseheld: None,
            mouserelease: None,
        }
    }
    fn isinside(&self, (x, y): (u16, u16)) -> bool {
        x >= self.c1.0 && x <= self.c2.0 && y >= self.c1.1 && y <= self.c2.1
    }
    pub fn pressed(&mut self, mousebutton: MouseButton) -> bool {
        if let Some(mb) = self.mousepress {
            self.mousepress = None;
            mb == mousebutton
        } else {
            false
        }
    }
    pub fn held(&self, mousebutton: MouseButton) -> bool {
        if let Some(mb) = self.mouseheld {
            mb == mousebutton
        } else {
            false
        }
    }
    pub fn released(&mut self, mousebutton: MouseButton) -> bool {
        if let Some(mb) = self.mouserelease {
            self.mouserelease = None;
            mb == mousebutton
        } else {
            false
        }
    }
}

impl GameObject for Button {
    fn input(&mut self, e: &Event) {
        match e {
            Event::Mouse(m) => match m {
                MouseEvent::Press(b, x, y) => {
                    if self.isinside((*x, *y)) {
                        self.mousepress = Some(*b);
                        self.mouseheld = Some(*b);
                    }
                    self.mouserelease = None;
                }
                MouseEvent::Release(x, y) => {
                    if let Some(b) = self.mouseheld {
                        if self.isinside((*x, *y)) {
                            self.mouserelease = Some(b);
                        }
                    }
                    self.mouseheld = None;
                    self.mousepress = None;
                }
                _ => (),
            },
            _ => (),
        }
    }
    fn render(&mut self, buff: &mut Vec<u8>) {
        write!(buff, "{}", cursor::Goto(self.c1.0, self.c1.1)).unwrap();
        buff.extend(self.col.iter());
        for _ in self.c1.1..self.c2.1 + 1 {
            for _ in self.c1.0..self.c2.0 + 1 {
                if self.mouseheld.is_some() {}
                write!(
                    buff,
                    "{}",
                    if !self.mouseheld.is_some() {
                        '█'
                    } else {
                        '▒'
                    }
                )
                .unwrap();
            }
            write!(buff, "\n{}", cursor::Left(self.c2.0 - self.c1.0 + 1))
                .unwrap();
        }
    }
}
