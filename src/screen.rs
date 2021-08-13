use std::io::{Stdin, Stdout, Write, stdin, stdout};
use crate::vec2::{Vec2, Rect};
use termion::{input::{Events, MouseTerminal, TermRead}, raw::{IntoRawMode, RawTerminal}, screen::AlternateScreen, terminal_size};

type Input = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
    }

    pub fn bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Color,
}

pub struct Screen {
    // Size of Screen.
    pub size: Vec2<usize>,

    // Modified portion of Screen.
    pub mods: Option<Rect<usize>>,

    // The buffer of Screen data. 
    pub chrs: Vec<(char, Style)>,

    pub out: Input
}

impl Screen {
    pub fn new() -> Screen {
        // Init the stdout
        let out = stdout().into_raw_mode().unwrap();
        let out = AlternateScreen::from(out);
        let mut out = MouseTerminal::from(out);
        
        // Hide the cursor.
        out.write(termion::cursor::Hide.to_string().as_bytes()).unwrap();
        out.flush().unwrap();

        // Get the size of the terminal.
        let size = {
            let (x, y) = terminal_size().unwrap();
            Vec2::new(x as usize, y as usize)
        };

        let style = Style {
            fg: Color(0, 0, 0)
        };

        return Screen {
            mods: None, 
            chrs: vec![(' ', style) ; size.x * size.y],
            size,
            out,
        };
    }

    pub fn set(&mut self, chr: char, fg: Color, cord: &Vec2<usize>) {
        self.chrs[self.size.index(cord)] = (chr, Style { fg });
        self.fit(cord);
    }

    pub fn fit(&mut self, cord: &Vec2<usize>) {
        if let Some(mods) = self.mods {
            self.mods = Some(mods.fit(cord));
        } else {
            self.mods = Some(Rect(cord.clone(), cord.clone())); 
        }
    }

    pub fn blit(&mut self) {
        if let Some(mods) = self.mods {
            for y in mods.0.y..mods.1.y + 1 { 
                // position the cursor at the start of the new line
                self.out.write(format!("\x1b[{};{}H", y + 1, 1).as_bytes()).unwrap();

                for x in mods.0.x..mods.1.x + 1 {

                    // add the character to the line
                    let (chr, style) = self.chrs[ self.size.index(&Vec2::new(x, y)) ];
                    self.out.write(style.fg.fg().as_bytes()).unwrap();
                    self.out.write(&[chr as u8]).unwrap();
                }
            }
        }

        self.mods = None;
        self.out.flush().unwrap();
    }

    pub fn events(&self) -> Events<Stdin> {
        return stdin().events();
    }

    pub fn show_cursor(&mut self) {
        self.out.write(termion::cursor::Show.to_string().as_bytes()).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        self.out.write(termion::cursor::Hide.to_string().as_bytes()).unwrap();
    }

    pub fn move_cursor(&mut self, cord: &Vec2<usize>) {
        self.out.write(termion::cursor::Goto((cord.x - 1) as u16, (cord.y - 1) as u16).to_string().as_bytes()).unwrap();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.show_cursor();
        self.out.flush().unwrap();
    }
}
