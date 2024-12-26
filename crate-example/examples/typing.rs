use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use std::fs::read_to_string;
use std::io::stdout;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use ansi_term::Color::Red;

struct App {
    // File Content
    file_content: String,
    // User input
    user_input: String,
}

impl App {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?;
        Ok(Self {
            file_content,
            user_input: String::new(),
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new("data/typing.txt")?;

    loop {
        println!("{}", app.file_content);
        for (user_input_char, target_char) in
            app.user_input.chars().zip(app.file_content.chars().cycle()) {
            if user_input_char == target_char {
                print!("{target_char}");
            } else {
                print!("{}", Red.paint("*"));
            }
        }
        println!("_");

        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => app.user_input.push(c),
                    KeyCode::Enter => {
                        let total_chars = app.file_content.chars().count();
                        let total_right = app.user_input.chars()
                            .zip(app.file_content.chars())
                            .filter(|(a, b)| a == b)
                            .count();
                        println!("You got {total_right} out of {total_chars}!");
                        // When user press Enter key, print the typing result and exit the loop.
                        return Ok(());
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All));
        }
    }
    Ok(())
}
