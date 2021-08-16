///
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB(pub u8, pub u8, pub u8);

///
#[derive(Default, Clone, PartialEq, Eq)]
pub struct FgColor(pub RGB);

impl Style for FgColor {
    fn to_cmd(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0.0, self.0.1, self.0.2)
    }
}

///
#[derive(Clone, PartialEq, Eq)]
pub struct Color {
    pub fg: RGB,
    pub bg: RGB,
}

impl Style for Color {
    fn to_cmd(&self) -> String {
        // The foreground color
        format!("\x1b[38;2;{};{};{}m", self.fg.0, self.fg.1, self.fg.2) +
        
        
        // The background color
        format!("\x1b[48;2;{};{};{}m", self.bg.0, self.bg.1, self.bg.2).as_str()
    }
}

impl Default for Color {
    fn default() -> Self {
        return Color {
            fg: RGB(200, 200, 200),
            bg: RGB(0, 0, 0),
        }
    }
}

///


///
pub trait Style {
    fn to_cmd(&self) -> String;
}

