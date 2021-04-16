use termion::event::*;
use termion::terminal_size;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::screen::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::io::{stdout, stdin, Write};

use crate::term::*;

pub struct Term {
    pub handle: JoinHandle<()>,
    pub events: Receiver<Event>,
    pub screen: Arc<Mutex<Screen>>,
    pub size: Vec2,

    alive: Arc<AtomicBool>,
}

enum Killabe<Event> {
    Kill,
    Event(Event)
}

fn input_thread(sender: Sender<Killabe<Event>>) -> JoinHandle<()> {
    thread::spawn(move || {
        for event in stdin().events() {
            if let Ok(event) = event {
                if let Err(_) = sender.send(Killabe::<Event>::Event(event)) {
                    break;
                }
            }
        }
    })
}

fn killable_channel() -> (Sender<Killabe<Event>>, Receiver<Event>, JoinHandle<()>) { 
    let (emiter, recver) = channel::<Killabe<Event>>();
    let (emiter2, recver2) = channel::<Event>();

    let handle = thread::spawn(move || {
        for event in recver.iter() {
            match event {
                Killabe::Kill => break,
                Killabe::<Event>::Event(event) => emiter2.send(event).unwrap(),
            }
        }
    });

    return (emiter, recver2, handle);
}

impl Term {
    pub fn new() -> Term {
        let (sender, recver, killable_channel_handle) = killable_channel();

        let size = {
            let (x, y) = terminal_size().unwrap();
            (x as isize, y as isize).into()
        };

        let screen = Screen::new(size);

        let alive = Arc::new(AtomicBool::new(true)); 

        let main_handle = {
            let alive = alive.clone();
            let screen = screen.clone();

            thread::spawn(move || {
                let mut buffer = {
                    let stdout = stdout().into_raw_mode().unwrap();
                    let stdout = AlternateScreen::from(stdout);
                    let stdout = MouseTerminal::from(stdout);
                    stdout.into_raw_mode().unwrap()
                };

                input_thread(sender.clone());

                while alive.load(Ordering::Relaxed) {
                    let output = screen.lock().unwrap().render();
                    buffer.write(output.as_bytes()).unwrap();
                    buffer.flush().unwrap();
                    thread::sleep(Duration::from_millis(100));
                }

                sender.send(Killabe::<Event>::Kill).unwrap();

                // make sure all the outher threads exit gracfully
                killable_channel_handle.join().unwrap()
            })
        };

        return Term {
            events: recver,
            handle: main_handle,
            screen,
            alive,
            size,
        };
    }

    pub fn kill(&self) {
        self.alive.swap(false, Ordering::Relaxed);
    }
}

impl Screen {
    fn render(&self) -> String {
        let minimum_screen_size = self.size.x * self.size.y;
        let mut out = String::with_capacity(minimum_screen_size as usize);

        let mut curr_fg = self.fg.get(&(0, 0).into());
        let mut curr_bg = self.bg.get(&(0, 0).into());

        out += curr_fg.fg_cmd().as_str();
        out += curr_bg.bg_cmd().as_str();

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let cord = &(x, y).into();

                let fg = self.fg.get(cord);
                if fg != curr_fg {
                    out += fg.fg_cmd().as_str();
                    curr_fg = fg;
                }

                let bg = self.bg.get(cord);
                if bg != curr_bg {
                    out += bg.bg_cmd().as_str();
                    curr_bg = bg;
                }

                out.push(self.chars.get(cord).clone());
            }
        }

        return out;
    } 
}

impl Color {
    fn fg_cmd(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
    }

    fn bg_cmd(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
    }
}
