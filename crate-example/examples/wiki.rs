use std::io::stdout;
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::execute;
use crossterm::terminal::{ClearType, Clear};
use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Default)]
struct App {
    current_article: CurrentArticle,
    search_string: String,
}

impl App {
    fn get_article(&mut self) -> Result<(), Box<dyn Error>> {
        let text = get(format!("{URL}/{}", self.search_string))?.text()?;
        let as_article: CurrentArticle = serde_json::from_str(&text)?;
        self.current_article = as_article;
        Ok(())
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "    Searching for: {}
Title: {}
----------------
Description: {}
----------------
{}", self.search_string,
            self.current_article.title,
            self.current_article.description,
            self.current_article.extract
        )
    }
}

#[derive(Debug, Deserialize, Default)]
struct CurrentArticle {
    title: String,
    description: String,
    extract: String,
}

const URL: &str = "https://en.wikipedia.org/api/rest_v1/page/summary";

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();

    loop {
        println!("{app}");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.search_string.pop();
                        println!("{}", app.search_string);
                    }
                    KeyCode::Esc => app.search_string.clear(),
                    KeyCode::Enter => app.get_article()?,
                    KeyCode::Char(c) => {
                        app.search_string.push(c);
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All)).unwrap();
        }
    }
    Ok(())
}