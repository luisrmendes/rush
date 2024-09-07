use crate::{commands, GlobalState, Systems};
use crossterm::event::{self, Event, KeyCode};
use log::warn;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self, execute,
        terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub struct Tui {
    global_state: Arc<Mutex<GlobalState>>,
    systems: Systems,
}

impl Tui {
    pub fn new(global_state: Arc<Mutex<GlobalState>>, systems: Systems) -> Self {
        Self {
            global_state,
            systems,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(std::io::stdout(), Clear(ClearType::All))?;
        execute!(io::stdout(), EnterAlternateScreen)?;

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            let brightness = self.global_state.lock().await.office_env.brightness;
            let temp = self.global_state.lock().await.office_env.temperature;
            let humidity = self.global_state.lock().await.office_env.humidity;
            let am_i_home = self.global_state.lock().await.am_i_home;
            let sys1_is_online = if let Ok(out) = commands::is_online(&self.systems.pcs[0]) {
                out
            } else {
                warn!("Falied to get is_online");
                false
            };

            // Draw the terminal UI
            terminal.draw(|f| {
                let size = f.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(20), // For progress bar
                        Constraint::Percentage(20), // For value rectangles
                        Constraint::Percentage(20), // For value rectangles
                        Constraint::Percentage(20), // For value rectangles
                        Constraint::Percentage(20), // For value rectangles
                    ])
                    .split(size);

                // // Draw the progress bar
                // let progress_value = 0.2;
                // let gauge = Gauge::default()
                //     .block(Block::default().borders(Borders::ALL).title("Progress"))
                //     .gauge_style(Style::default().fg(Color::Green))
                //     .ratio(progress_value);
                // f.render_widget(gauge, chunks[0]);

                // Draw the value rectangles
                let inner_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                        Constraint::Percentage(10), // For value rectangles
                    ])
                    .split(chunks[3]);

                let value1_paragraph = Paragraph::new(format!("{brightness}"))
                    .block(Block::default().borders(Borders::ALL).title("Brightness"));
                let value2_paragraph = Paragraph::new(format!("{temp}"))
                    .block(Block::default().borders(Borders::ALL).title("Temperature"));
                let value3_paragraph = Paragraph::new(format!("{humidity}"))
                    .block(Block::default().borders(Borders::ALL).title("Humidity"));
                let value4_paragraph = Paragraph::new(format!("{am_i_home}"))
                    .block(Block::default().borders(Borders::ALL).title("Am I home?"));
                let value5_paragraph = Paragraph::new(format!("{sys1_is_online}")).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Snowdog Status"),
                );

                f.render_widget(value1_paragraph, inner_chunks[0]);
                f.render_widget(value2_paragraph, inner_chunks[1]);
                f.render_widget(value3_paragraph, inner_chunks[2]);
                f.render_widget(value4_paragraph, inner_chunks[3]);
                f.render_widget(value5_paragraph, inner_chunks[4]);
            })?;

            // Handle user input to exit the loop
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        terminal.clear()?;
                        terminal.show_cursor()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        execute!(std::io::stdout(), Clear(ClearType::All))?;
                        std::process::exit(0);
                    }
                }
            }
        }
    }
}
