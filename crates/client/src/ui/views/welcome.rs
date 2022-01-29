use super::*;

#[derive(Debug)]
pub enum Flow {
    Redraw,
    Enter,
}

#[derive(Debug)]
pub struct Welcome {
    pub config: Config,
    pub input:  Input,
}

impl Welcome {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            input: Input::new("Name: ", "<anon>", ""),
        }
    }

    pub fn render<W: Write>(&self, mut w: W) {
        self.input.render(1, 1, self.config, w);
    }

    pub fn handle(&mut self, event: x::Event) -> Option<Flow> {
        if let Some(handled) = self.input.handle(event) {
            Some(match handled {
                input::Flow::Redraw => Flow::Redraw,
                input::Flow::Enter => Flow::Enter,
            })
        } else {
            None
        }
    }
}
