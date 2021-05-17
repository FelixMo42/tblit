# Tblit #
Tblit is a pure rust library for dealing with alternate terminal buffer.

When you initliaze the terminal it creats an alternate thread to deal with io.
When you want to draw lock down the surface and draw to it and once its released the thread while draw it to the terminal.

Supports any ANSI terminal.

## Usage ##
```rust
pub use tblit::*;
pub use tblit::event::{Event, MouseEvent};

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
```
