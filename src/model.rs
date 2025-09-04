use crate::update;
use crate::view::render;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| render(frame))?;
            match event::read()? {
                // NOTE: only support keyboard for now
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    update::on_key_event(&mut self, key)
                }
                _ => {}
            }
        }
        Ok(())
    }
}
