mod config;
// mod views;
// mod widgets;

pub use config::*;
// pub use views::*;
// pub use widgets::*;

use super::*;
use futures::future::ready;
use futures::FutureExt;
use futures::StreamExt;
use std::io::stdout;
use std::io::StdoutLock;
use std::io::Write;

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
    pub use crossterm::event::KeyEvent;
    pub use crossterm::event::KeyModifiers;
    pub use crossterm::event::MouseEvent;
    pub use crossterm::event::MouseEventKind;
    pub use crossterm::execute;
    pub use crossterm::queue;
    pub use crossterm::style::Attribute;
    pub use crossterm::style::Color;
    pub use crossterm::style::SetAttribute;
    pub use crossterm::style::SetForegroundColor;
    pub use crossterm::style::Stylize;
    pub use crossterm::terminal::disable_raw_mode;
    pub use crossterm::terminal::enable_raw_mode;
    pub use crossterm::terminal::size;
    pub use crossterm::terminal::Clear;
    pub use crossterm::terminal::ClearType;
    pub use crossterm::terminal::EnterAlternateScreen;
    pub use crossterm::terminal::LeaveAlternateScreen;
}

pub fn enter() {
    x::enable_raw_mode().unwrap();
    x::execute!(
        stdout(),
        x::EnableMouseCapture,
        x::EnterAlternateScreen,
        x::Hide
    )
    .unwrap();
}

pub fn leave() {
    x::execute!(
        stdout(),
        x::DisableMouseCapture,
        x::LeaveAlternateScreen,
        x::Show
    )
    .unwrap();
    x::disable_raw_mode().unwrap();
}

pub fn exit() {
    leave();
    std::process::exit(0);
}

#[derive(Debug)]
pub enum Flow {
    Redraw,
    Submit,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl Rect {
    pub fn shrink_center(mut self, i: u16) -> Rect {
        self.x += i;
        self.y += i;
        self.w -= 2 * i;
        self.h -= 2 * i;
        self
    }

    pub fn up(&self, rows: u16) -> Self {
        let mut rect = *self;
        rect.x -= rows;
        rect
    }

    pub fn down(&self, rows: u16) -> Self {
        let mut rect = *self;
        rect.x += rows;
        rect
    }

    pub fn move_to<W: Write>(&self, mut w: W) {
        x::queue!(&mut w, x::MoveTo(self.x, self.y));
    }
}

impl From<(u16, u16, u16, u16)> for Rect {
    fn from((x, y, w, h): (u16, u16, u16, u16)) -> Self {
        Self { x, y, w, h }
    }
}

pub trait View: Sized {
    // type Props: Default;
    // type State: Default;

    // fn new(config: Config) -> Self;
    // fn set_props(&mut self, props: Self::Props);
    fn render<W: Write>(&self, w: W) {}
    // fn handle(&mut self, event: x::Event) -> Option<Flow>;

    // fn with_props(config: Config, props: Self::Props) -> Self {
    // let mut view = Self::new(config);
    // view.set_props(props);
    // view
    // }
}

trait WriteExt: Write + Sized {
    fn clear(&mut self) {
        x::queue!(self, x::Clear(x::ClearType::All));
    }

    fn move_to(&mut self, x: u16, y: u16) {
        x::queue!(self, x::MoveTo(x, y));
    }

    fn render<V: View>(mut self: &mut Self, view: &V) {
        self.clear();
        view.render(&mut self);
        self.flush();
    }
}

impl<W: Write> WriteExt for W {}

pub async fn main() -> Option<()> {
    let out = stdout();
    let mut out = out.lock();

    let config = Config::new();
    let db = fake::db();
    let state = State {
        config,
        db,
        input: "lol".into(),
    };

    state.render(&mut out);
    out.flush();

    None
}

#[derive(Debug)]
pub struct State {
    config: Config,
    db:     Db,
    input:  String,
}

impl State {
    pub fn render<W: Write>(&self, mut w: W) {
        let width = self.config.width;
        let height = self.config.height;
        let input_h = 3;
        let right_x = 0;
        let right_w = width - right_x;

        let right_rect: Rect = (right_x, 0, right_w, height).into();
        let input_rect: Rect = (right_x, height - input_h, right_w, input_h).into();
        let events_rect: Rect = (right_x, 0, right_w, right_rect.h - input_h).into();

        self.render_input(&mut w, input_rect);
        self.render_events(&mut w, events_rect);
    }

    pub fn render_events<W: Write>(&self, mut w: W, rect: Rect) {
        self.render_borders(&mut w, rect);
        let rect = rect.shrink_center(1);

        let scrollbar_rect: Rect = (rect.x + rect.w - 1, rect.y, 1, rect.h).into();
        let rect: Rect = (rect.x, rect.y, rect.w - 2, rect.h).into();

        let events = self.db.world().iter();
        let lines = events
            .map(|event| fmt_channel_event(event, rect.w, self.config))
            .flatten()
            .rev()
            .collect::<Vec<_>>();

        for (i, line) in lines.iter().take(rect.h as usize).enumerate() {
            w.move_to(rect.x, rect.y + rect.h - (i as u16 + 1));
            write!(w, "{line}");
        }

        let t = lines.len();
        let h = rect.h as usize;
        if let Some(y) = t.checked_sub(h) {
            self.render_scrollbar(&mut w, scrollbar_rect, y, h, t);
        }
    }

    pub fn render_input<W: Write>(&self, mut w: W, rect: Rect) {
        self.render_borders(&mut w, rect);
        let rect = rect.shrink_center(1);
        let green = x::SetForegroundColor(self.config.colors.green);
        let yellow = x::SetForegroundColor(self.config.colors.yellow);
        let bold = x::SetAttribute(x::Attribute::Bold);
        let reset = x::SetAttribute(x::Attribute::Reset);
        let input = &self.input;

        w.move_to(rect.x, rect.y);
        write!(&mut w, "{green}{bold}>{reset} {yellow}{input}{reset}");
    }

    pub fn render_borders<W: Write>(&self, mut w: W, rect: Rect) {
        use x::Stylize;

        let color = self.config.colors.purple;
        let v = '│'.with(color);
        let h = '─'.with(color);
        let tl = '╭'.with(color);
        let tr = '╮'.with(color);
        let bl = '╰'.with(color);
        let br = '╯'.with(color);

        debug_assert!(rect.w > 1);
        debug_assert!(rect.h > 1);

        let render_h = |w: &mut W, y: u16, left, right| {
            write!(w, "{}{left}", x::MoveTo(rect.x, y));
            for _ in 2..rect.w {
                write!(w, "{h}");
            }
            write!(w, "{right}");
        };
        let render_v = |w: &mut W, y: u16| {
            let left = x::MoveTo(rect.x, y);
            let right = x::MoveTo(rect.x + rect.w, y);
            write!(w, "{left}{v}{right}{v}");
        };

        render_h(&mut w, rect.y, tl, tr);
        (2..rect.h).for_each(|i| render_v(&mut w, rect.y + i - 1));
        render_h(&mut w, rect.y + rect.h - 1, bl, br);
    }

    pub fn render_scrollbar<W: Write>(&self, mut w: W, rect: Rect, y: usize, h: usize, t: usize) {
        use x::Stylize;

        const INACTIVE: char = '│';
        const ACTIVE: char = '┃';

        let inactive = INACTIVE.with(self.config.colors.purple);
        let active = ACTIVE.with(self.config.colors.pink);

        let h = (h as f32 * rect.h as f32 / t as f32).round() as u16;
        let y = (y as f32 * rect.h as f32 / t as f32).round() as u16;
        let y0 = rect.y;
        let y1 = rect.y + y;
        let y2 = rect.y + y + h;
        let y3 = rect.y + rect.h;

        for (range, char) in [
            ((y0..y1), inactive),
            ((y1..y2), active),
            ((y2..y3), inactive),
        ] {
            for i in range {
                w.move_to(rect.x, i);
                write!(&mut w, "{char}");
            }
        }
    }
}

fn fmt_channel_event(event: &ChannelEvent, width: u16, config: Config) -> Vec<String> {
    let bold = x::SetAttribute(x::Attribute::Bold);
    let no_bold = x::SetAttribute(x::Attribute::NoBold);
    let italic = x::SetAttribute(x::Attribute::Italic);
    let reset = x::SetAttribute(x::Attribute::Reset);
    let fg = |color| x::SetForegroundColor(color);
    let yellow = fg(config.colors.yellow);
    let purple = fg(config.colors.purple);
    let pink = fg(config.colors.pink);

    let fmt_event =
        |name, event| format!("{italic}{purple}**{bold}{name}{no_bold} {event}**{reset}");
    let fmt_message =
        |name, message| format!("{pink}{bold}[{name}]{reset} {yellow}{message}{reset}");
    let fmt_lines = |(i, str): (_, std::borrow::Cow<str>)| {
        if i == 0 {
            str.into_owned()
        } else {
            format!("{yellow}{str}{reset}")
        }
    };

    let fmt = match event {
        ChannelEvent::Enter { user } => fmt_event(&user.name, "entered"),
        ChannelEvent::Leave { user } => fmt_event(&user.name, "left"),
        ChannelEvent::Post { user, message } => fmt_message(&user.name, &message.body),
    };

    textwrap::wrap(&fmt, width as usize)
        .into_iter()
        .enumerate()
        .map(fmt_lines)
        .collect()
}

/*
pub async fn main2(mut client: Client) -> Option<()> {
    let out = stdout();
    let mut out = out.lock();

    let config = Config::new();
    let quit = x::KeyEvent {
        code:      x::KeyCode::Char('c'),
        modifiers: x::KeyModifiers::CONTROL,
    };

    let mut stream = x::EventStream::new();
    let mut stream = Box::pin(
        stream
            .filter_map(|event| async move { event.ok() })
            .take_while(|event| ready(*event != x::Event::Key(quit))),
    );

    let mut welcome_view = WelcomeView::new(config);
    out.render(&welcome_view);

    let mut name: Option<String> = None;

    while let Some(event) = stream.next().await {
        match welcome_view.handle(event) {
            Some(Flow::Redraw) => {
                out.render(&welcome_view);
            }
            Some(Flow::Submit) => {
                name = Some(welcome_view.input.value().into());
                break;
            }
            _ => {}
        }
    }

    let name = name?;

    let mut connection = client.connect_user(name.clone()).await;
    connection.enter_world().await;

    let mut channel_view = ChannelView::new(config);
    out.render(&channel_view);

    while let Some(event) = stream.next().await {
        match channel_view.handle(event) {
            Some(Flow::Redraw) => {
                out.render(&channel_view);
            }
            Some(Flow::Submit) => {
                let message: String = channel_view.input.value().into();
                channel_view.input.clear();

                out.render(&channel_view);
            }
            _ => {}
        }
    }

    Some(())
}
*/
