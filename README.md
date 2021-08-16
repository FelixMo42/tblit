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
    let term = Screen::new(Color {
        fg: RGB(200, 200, 200),
        bg: RGB(30, 30, 30),
    });

    for event in term.events.iter() {
        match event {
            Event::Mouse(MouseEvent::Release(x, y)) => {
                screen.set(' ', Color {
                    fg: RGB(200, 200, 200),
                    bg: RGB(100, 100, 200)
                });

                screen.blit();
            },
            Event::Mouse(_) => {},
            _ => break,
        }
    }
}
```
