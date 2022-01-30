use super::*;
use textwrap::wrap;
use textwrap::Options;

#[derive(Default, Debug)]
pub struct MessageProps {
    x:       u16,
    y:       u16,
    width:   u16,
    message: String,
}

#[derive(Debug)]
pub struct Message {
    pub config: Config,
    pub lines:  Vec<String>,
    pub rect:   Rect,
}

impl Message {
    fn wrap(message: String, width: u16) -> Vec<String> {
        wrap(&message, width as usize)
            .into_iter()
            .map(|str| str.into_owned())
            .collect()
    }
}

impl View for Message {
    type Props = MessageProps;
    type State = ();

    fn new(config: Config) -> Self {
        Self {
            config,
            lines: Vec::new(),
            rect: Rect::default(),
        }
    }

    fn set_props(&mut self, props: Self::Props) {
        self.lines = Message::wrap(props.message, props.width);
        self.rect.x = props.x;
        self.rect.y = props.y;
        self.rect.w = props.width;
        self.rect.h = self.lines.len() as u16;
    }

    fn render<W: Write>(&self, mut w: W) {
        self.rect.move_to(&mut w);
        for (i, line) in self.lines.iter().enumerate() {
            self.rect.down(i as u16).move_to(&mut w);
            write!(&mut w, "{line}");
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct MessageListProps {
    pub rect:     Rect,
    pub messages: Vec<String>,
}

#[derive(Debug)]
pub struct MessageList {
    pub config:   Config,
    pub props:    MessageListProps,
    pub rect:     Rect,
    pub messages: Vec<Vec<String>>,
}

impl View for MessageList {
    type Props = MessageListProps;
    type State = ();

    fn new(config: Config) -> Self {
        Self {
            config,
            props: Default::default(),
            rect: Default::default(),
            messages: Vec::new(),
        }
    }

    fn set_props(&mut self, props: Self::Props) {
        self.rect = props.rect;
        self.props = props.clone();
        self.messages = props
            .messages
            .into_iter()
            .map(|message| Message::wrap(message, self.rect.w))
            .collect();
    }

    fn render<W: Write>(&self, mut w: W) {}
}
