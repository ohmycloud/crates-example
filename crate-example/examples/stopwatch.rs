use std::io::stdout;
use std::time::Instant;
use chrono::Utc;
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crate::StopwatchState::*;

const ONE_MINUTE: u128 = 60 * 1000;
const ONE_SECOND: u128 = 1000;

enum StopwatchState {
    NotStarted,
    Running,
    Done,
}

struct Stopwatch {
    now: Instant,
    state: StopwatchState,
    display: String,
}

impl Stopwatch {
    fn new() -> Self {
        Self {
            now: Instant::now(),
            state: NotStarted,
            display: String::from("0:00:00"),
        }
    }

    fn get_time(&self) -> String {
        match self.state {
            NotStarted => String::from("0:00:00"),
            Running => {
                let mut elapsed = self.now.elapsed().as_millis();
                let minutes = elapsed / ONE_MINUTE;
                elapsed -= minutes * ONE_MINUTE;
                let seconds = elapsed / ONE_SECOND;
                elapsed -= seconds * ONE_SECOND;
                let split_seconds = elapsed / 10;

                format!("{minutes}:{seconds}:{split_seconds}")
            }
            Done => self.display.clone(),
        }
    }

    fn next_state(&mut self) {
        match self.state {
            NotStarted => {
                self.now = Instant::now();
                self.state = Running;
            }
            Running => {
                self.display = self.get_time();
                self.state = Done;
            }
            Done => self.state = NotStarted,
        }
    }
}

fn block_with(input: &str) -> Block {
    Block::default().title(input).borders(Borders::ALL)
}

fn utc_pretty() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() -> Result<(), anyhow::Error> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    // The ratatui Terminal takes a crossterm backend
    let mut terminal = Terminal::new(backend)?;
    let mut stopwatch = Stopwatch::new();

    loop {
        if poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key_event) = read()? {
                if let (KeyCode::Enter, KeyEventKind::Press) = (key_event.code, key_event.kind) {
                    stopwatch.next_state();
                }
            }
        }

        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(f.size());

            let stopwatch_area = layout[0];
            let utc_time_area = layout[1];
            let stopwatch_block = block_with("Stopwatch");
            let utc_time_block = block_with("UTC time in London");

            let stopwatch_text = Paragraph::new(stopwatch.get_time()).block(stopwatch_block);
            let utc_text = Paragraph::new(utc_pretty()).block(utc_time_block);

            f.render_widget(stopwatch_text, stopwatch_area);
            f.render_widget(utc_text, utc_time_area);
        }).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        terminal.clear().unwrap();
    }
}