use std::{error::Error, io::Write};

use termion::{
    color, cursor,
    event::{Event, MouseButton, MouseEvent},
};

use termion_game_engine::{col2fg_str, GameObject};

pub const BOX_THIN: &[char] = &[
    ' ', '╶', '╴', '─', '╷', '└', '┘', '┴', '╵', '┌', '┐', '┬', '│', '├', '┤',
    '┼',
];

pub const BOX_THIC: &[char] = &[
    ' ', '╺', '╸', '━', '╹', '┗', '┛', '┻', '╻', '┏', '┓', '┳', '┃', '┣', '┫',
    '╋',
];

pub const BOX_MIX: &[char] = &[
    ' ', '╶', '╺', '╴', '─', '╼', '╸', '╾', '━', '╷', '└', '┕', '┘', '┴', '┶',
    '┙', '┵', '┷', '╹', '┖', '┗', '┚', '┸', '┺', '┛', '┹', '┻', '╵', '┌', '┍',
    '┐', '┬', '┮', '┑', '┭', '┯', '│', '├', '┝', '┤', '┼', '┾', '┥', '┽', '┿',
    '╿', '┞', '┡', '┦', '╀', '╄', '┩', '╃', '╇', '╻', '┎', '┏', '┒', '┰', '┲',
    '┓', '┱', '┳', '╽', '┟', '┢', '┧', '╁', '╆', '┪', '╅', '╈', '┃', '┠', '┣',
    '┨', '╂', '╊', '┫', '╉', '╋',
];

pub fn box_mix(dirs: [Option<bool>; 4]) -> char {
    BOX_MIX[dirs
        .iter()
        .fold((0usize, 1usize), |(a, b), &x| {
            (
                a + if let Some(x) = x { 1 + x as usize } else { 0 } * b,
                b * 3,
            )
        })
        .0]
}

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
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Button {
            c1: pos,
            c2: (pos.0 + size.0 - 1, pos.1 + size.1 - 1),
            col: col2fg_str(col)?,
            mousepress: None,
            mouseheld: None,
            mouserelease: None,
        })
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
    fn render(&mut self, buff: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        write!(buff, "{}", cursor::Goto(self.c1.0, self.c1.1))?;
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
                )?;
            }
            write!(buff, "\n{}", cursor::Left(self.c2.0 - self.c1.0 + 1))?;
        }
        write!(buff, "{}", color::Fg(color::Reset))?;
        Ok(())
    }
}

pub struct TextLabel {
    pos: (u16, u16),
    text: String,
    col: Vec<u8>,
}

impl TextLabel {
    pub fn new<T: color::Color>(
        pos: (u16, u16),
        text: String,
        col: T,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(TextLabel {
            pos: pos,
            text: text,
            col: col2fg_str(col)?,
        })
    }
}

impl GameObject for TextLabel {
    fn render(&mut self, buff: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        buff.extend(self.col.iter());
        write!(
            buff,
            "{}{}{}",
            cursor::Goto(self.pos.0, self.pos.1),
            self.text,
            color::Fg(color::Reset)
        )?;
        Ok(())
    }
}
