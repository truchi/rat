use std::io::stdout;
use std::io::StdoutLock;
use std::io::Write;

macro_rules! rgb {
    ($r:literal, $g:literal, $b:literal) => {
        x::Color::Rgb {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

mod x {
    pub use crossterm::cursor::position;
    pub use crossterm::cursor::Hide;
    pub use crossterm::cursor::MoveTo;
    pub use crossterm::cursor::Show;
    pub use crossterm::event::DisableMouseCapture;
    pub use crossterm::event::EnableMouseCapture;
    pub use crossterm::event::Event;
    pub use crossterm::event::EventStream;
    pub use crossterm::event::KeyCode;
    pub use crossterm::execute;
    pub use crossterm::queue;
    pub use crossterm::style::Color;
    pub use crossterm::terminal::disable_raw_mode;
    pub use crossterm::terminal::enable_raw_mode;
    pub use crossterm::terminal::size;
    pub use crossterm::terminal::EnterAlternateScreen;
    pub use crossterm::terminal::LeaveAlternateScreen;
}

#[derive(Copy, Clone, Debug)]
struct Colors {
    pub background: x::Color,
    pub current:    x::Color,
    pub foreground: x::Color,
    pub comment:    x::Color,
    pub cyan:       x::Color,
    pub green:      x::Color,
    pub orange:     x::Color,
    pub pink:       x::Color,
    pub purple:     x::Color,
    pub red:        x::Color,
    pub yellow:     x::Color,
}

const COLORS: Colors = Colors {
    background: rgb!(40, 42, 54),
    current:    rgb!(68, 71, 90),
    foreground: rgb!(248, 248, 242),
    comment:    rgb!(98, 114, 164),
    cyan:       rgb!(139, 233, 253),
    green:      rgb!(80, 250, 123),
    orange:     rgb!(255, 184, 108),
    pink:       rgb!(255, 121, 198),
    purple:     rgb!(189, 147, 249),
    red:        rgb!(255, 85, 85),
    yellow:     rgb!(241, 250, 140),
};

#[derive(Copy, Clone, Debug)]
struct Config {
    pub width:  u16,
    pub height: u16,
    pub colors: Colors,
}

impl Config {
    pub fn new() -> Self {
        let (width, height) = x::size().unwrap();

        Self {
            width,
            height,
            colors: COLORS,
        }
    }
}

#[derive(Debug)]
struct UserNameView {
    pub config:      Config,
    pub label:       String,
    pub placeholder: String,
    pub value:       String,
}

impl UserNameView {
    pub fn render<W: Write>(&self, mut w: W) {
        write!(w, "{}{}: ", x::MoveTo(1, 1), self.label);

        if self.value.is_empty() {
            write!(w, "{}", self.placeholder);
        } else {
            write!(w, "{}", self.value);
        }
    }
}

pub fn enter() {
    x::enable_raw_mode().unwrap();
    x::execute!(
        stdout(),
        x::EnableMouseCapture,
        x::EnterAlternateScreen,
        // x::Hide
    )
    .unwrap();
}

pub fn leave() {
    x::execute!(
        stdout(),
        x::DisableMouseCapture,
        x::LeaveAlternateScreen,
        // x::Show
    )
    .unwrap();
    x::disable_raw_mode().unwrap();
}

pub fn main() {
    let out = stdout();
    let mut out = out.lock();

    let config = Config::new();
    let user_name_view = UserNameView {
        config,
        label: "Name".into(),
        placeholder: "<anon>".into(),
        value: "".into(),
    };

    user_name_view.render(&mut out);
    out.flush();

    std::thread::sleep(std::time::Duration::from_secs(1));
}
