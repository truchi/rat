use super::*;

#[derive(Default, Debug)]
pub struct ChannelViewProps {
    pub messages: Vec<String>,
}

#[derive(Debug)]
pub struct ChannelView {
    pub config:       Config,
    // pub rect:         Rect,
    pub message_list: MessageList,
    // pub input:        Input,
}

impl ChannelView {
    // fn handle(&mut self, event: x::Event) -> Option<Flow> {
    // if let Some(handled) = self.input.handle(event) {
    // Some(match handled {
    // Flow::Redraw => Flow::Redraw,
    // Flow::Submit => Flow::Submit,
    // })
    // } else {
    // None
    // }
    // }
}

impl View for ChannelView {
    type Props = ChannelViewProps;
    type State = ();

    fn new(config: Config) -> Self {
        Self {
            config,
            message_list: MessageList::new(config),
            // input: Input::new("> ", "", ""),
        }
    }

    fn set_props(&mut self, props: Self::Props) {
        let props = MessageListProps {
            messages: props.messages,
            rect:     Rect::default(),
        };
        self.message_list.set_props(props);
    }

    fn render<W: Write>(&self, mut w: W) {
        // self.message_list.render(0, 0, self.config, &mut w);
        // self.input
        // .render(0, self.config.height - 1, self.config, &mut w);
    }
}
