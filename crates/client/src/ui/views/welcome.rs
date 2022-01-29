use super::*;

#[derive(Debug)]
pub struct WelcomeView {
    pub config: Config,
    pub input:  Input,
}

impl WelcomeView {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            input: Input::new("Name: ", "<anon>", ""),
        }
    }
}

impl View for WelcomeView {
    fn render<W: Write>(&self, mut w: W) {
        self.input.render(1, 1, self.config, w);
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
