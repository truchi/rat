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
    pub use crossterm::style::Color;
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

    let messages: Vec<String> = vec![
        "Hello, world!".into(),
        "Hello, world!".into(),
        "Hello, world!".into(),
        "Hello, world!".into(),
        "Hello, world!".into(),
        "Hello, world!".into(),
        "Hello, world!".into(),
        "Hello, world!".into(),
        r#"""
        Lorem ipsum dolor sit amet,
        consectetur adipiscing elit,
        sed do eiusmod tempor incididunt
        ut labore et dolore magna aliqua.
        Ut enim ad minim veniam, quis
        nostrud exercitation ullamco laboris
        nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit
        in voluptate velit esse cillum dolore
        eu fugiat nulla pariatur. Excepteur
        sint occaecat cupidatat non proident,
        sunt in culpa qui officia deserunt
        mollit anim id est laborum.
        """#
        .into(),
    ];

    // let channel_view_props = ChannelViewProps { messages };
    // let mut channel_view = ChannelView::with_props(config, channel_view_props);
    // out.render(&channel_view);

    None
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
