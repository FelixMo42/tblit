use std::io::{Stdin, Stdout, Write, stdin, stdout};
use crate::{style::Style, vec2::{Vec2, Rect}};
use termion::{input::{Events, MouseTerminal, TermRead}, raw::{IntoRawMode, RawTerminal}, screen::AlternateScreen, terminal_size};

type Input = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

pub struct Screen<T: Style + Clone> {
    // Size of Screen.
    pub size: Vec2<usize>,

    // Modified portion of Screen.
    pub mods: Option<Rect<usize>>,

    // The buffer of Screen data. 
    pub chrs: Vec<(char, T)>,

    pub out: Input
}

impl<T: Style + Clone> Screen<T> {
    pub fn new(default_style: T) -> Screen<T> {
        // Init the stdout
        let out = stdout().into_raw_mode().unwrap();
        let out = AlternateScreen::from(out);
        let mut out = MouseTerminal::from(out);
        
        // Hide the cursor.
        out.write(termion::cursor::Hide.to_string().as_bytes()).unwrap();
        out.flush().unwrap();

        // Get the size of the terminal.
        let (x, y) = terminal_size().unwrap();
        let (x, y) = (x as usize, y as usize);
        
        return Screen {
            mods: Some(Rect(Vec2::new(0, 0), Vec2::new(x - 1, y - 1))), 
            chrs: vec![(' ', default_style) ; x * y],
            size: Vec2::new(x, y),
            out,
        };
    }

    fn fit(&mut self, cord: &Vec2<usize>) {
        if let Some(mods) = self.mods {
            self.mods = Some(mods.fit(cord));
        } else {
            self.mods = Some(Rect(cord.clone(), cord.clone())); 
        }
    }

    pub fn set(&mut self, cord: &Vec2<usize>, chr: char, style: T) {
        self.chrs[self.size.index(cord)] = (chr, style);
        self.fit(cord);
    }

    pub fn blit(&mut self) {
        if let Some(mods) = self.mods {
            for y in mods.0.y..=mods.1.y { 
                // position the cursor at the start of the new line
                self.out.write(format!("\x1b[{};{}H", y + 1, 1).as_bytes()).unwrap();

                // add the characters to the line, with their style
                for x in mods.0.x..=mods.1.x {
                    let (chr, style) = &self.chrs[ self.size.index(&Vec2::new(x, y)) ];
                    self.out.write(style.to_cmd().as_bytes()).unwrap();
                    self.out.write(&[*chr as u8]).unwrap();
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
        self.out.write(termion::cursor::Goto((cord.x + 1) as u16, (cord.y + 1) as u16).to_string().as_bytes()).unwrap();
        self.out.flush().unwrap();
    }
}

impl<T: Style + Clone> Drop for Screen<T> {
    fn drop(&mut self) {
        self.show_cursor();
        self.out.flush().unwrap();
    }
}

