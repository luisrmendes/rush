use crate::GlobalState;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self, execute,
        terminal::{Clear, ClearType},
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};
use std::{io, sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub struct Tui {
    global_state: Arc<Mutex<GlobalState>>,
}

impl Tui {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self { global_state }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(std::io::stdout(), Clear(ClearType::All))?;

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            let brightness = self.global_state.lock().await.office_env.brightness;
            let temp = self.global_state.lock().await.office_env.temperature;
            let am_i_home = self.global_state.lock().await.am_i_home;

            // Draw the terminal UI
            terminal.draw(|f| {
                let size = f.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(30), // For progress bar
                        Constraint::Percentage(30), // For value rectangles
                        Constraint::Percentage(30), // For value rectangles
                    ])
                    .split(size);

                let progress_value = 0.2;

                // Draw the progress bar
                let gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Progress"))
                    .gauge_style(Style::default().fg(Color::Green))
                    .ratio(progress_value);
                f.render_widget(gauge, chunks[0]);

                // Draw the value rectangles
                let inner_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ])
                    .split(chunks[2]);

                let value1_paragraph = Paragraph::new(format!("{brightness}"))
                    .block(Block::default().borders(Borders::ALL).title("Brightness"));
                let value2_paragraph = Paragraph::new(format!("{temp}"))
                    .block(Block::default().borders(Borders::ALL).title("Temperature"));
                let value3_paragraph = Paragraph::new(format!("{am_i_home}"))
                    .block(Block::default().borders(Borders::ALL).title("Am I home?"));

                f.render_widget(value1_paragraph, inner_chunks[0]);
                f.render_widget(value2_paragraph, inner_chunks[1]);
                f.render_widget(value3_paragraph, inner_chunks[2]);
            })?;

            // Handle user input to exit the loop
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        // Cleanup the terminal
        terminal.clear()?;
        Ok(())
    }
}
