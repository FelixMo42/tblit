mod term;

pub use term::{Term, Color};
pub use term::event::{Event, MouseEvent};

fn main() {
    let term = Term::new();

    {
        let mut screen = term.screen.lock().unwrap();
        for x in 0..term.size.x {
            for y in 0..term.size.y {
                let spot = &(x, y).into();
                screen.set(spot, 'x');
                screen.set_fg(spot, Color(0, 70, 0));
            }
        }
    }

    for event in term.events.iter() {
        match event {
            Event::Mouse(MouseEvent::Release(x, y)) => {
                let mut screen = term.screen.lock().unwrap();
                screen.set(&(x as isize - 1, y as isize - 1).into(), ' ');
            },
            Event::Mouse(_) => {},
            _ => term.kill()
        }
    }

    term.handle.join().unwrap();
}
