mod config;
mod views;
mod widgets;

pub use config::*;
pub use views::*;
pub use widgets::*;

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

pub trait View {
    fn render<W: Write>(&self, w: W);
    fn handle(&mut self, event: x::Event) -> Option<Flow>;
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

pub async fn main(mut client: Client) -> Option<()> {
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
