use std::io::{Stdin, Stdout, Write, stdin, stdout};
use crate::vec2::{Vec2, Rect};
use termion::{input::{Events, MouseTerminal, TermRead}, raw::{IntoRawMode, RawTerminal}, screen::AlternateScreen, terminal_size};

type Input = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

pub struct Screen {
    // Size of Screen.
    pub size: Vec2<usize>,

    // Modified portion of Screen.
    pub mods: Option<Rect<usize>>,

    // The buffer of Screen data. 
    pub chrs: Vec<char>,

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

        return Screen {
            mods: None, 
            chrs: vec![' ' ; size.x * size.y],
            size,
            out,
        };
    }

    pub fn set(&mut self, chr: char, cord: &Vec2<usize>) {
        self.chrs[self.size.index(cord)] = chr;
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
                    let chr = self.chrs[ self.size.index(&Vec2::new(x, y)) ];
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
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.out.write(termion::cursor::Show.to_string().as_bytes()).unwrap();
        self.out.flush().unwrap();
    }
}
