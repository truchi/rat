use super::*;

#[derive(Debug)]
pub struct ChannelView {
    pub config: Config,
    pub input:  Input,
}

impl ChannelView {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            input: Input::new("> ", "", ""),
        }
    }
}

impl View for ChannelView {
    fn render<W: Write>(&self, mut w: W) {
        self.input.render(0, self.config.height - 1, self.config, w);
    }

    fn handle(&mut self, event: x::Event) -> Option<Flow> {
        if let Some(handled) = self.input.handle(event) {
            Some(match handled {
                Flow::Redraw => Flow::Redraw,
                Flow::Submit => Flow::Submit,
            })
        } else {
            None
        }
    }
}
