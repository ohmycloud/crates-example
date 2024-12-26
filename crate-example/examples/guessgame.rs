use axum::{extract::Path, routing::get};
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;
use std::sync::Mutex;

const RANDOM_WORDS: [&str; 6] = ["MB", "Windy", "Gomes", "Johnny", "Seoul", "Interesting"];
static GAME: Mutex<GameApp> = Mutex::new(GameApp {
    current_word: String::new(),
    right_guesses: vec![],
    wrong_guesses: vec![],
});


#[derive(Clone, Debug, Default)]
struct GameApp {
    current_word: String,
    right_guesses: Vec<char>,
    wrong_guesses: Vec<char>,
}

enum Guess {
    Right,
    Wrong,
    AlreadyGuessed,
}

impl GameApp {
    fn start(&mut self) {
        self.current_word = RANDOM_WORDS[fastrand::usize(..RANDOM_WORDS.len())].to_lowercase();
        self.right_guesses.clear();
        self.wrong_guesses.clear();
    }

    fn check_guess(&self, guess: char) -> Guess {
        if self.right_guesses.contains(&guess) ||
            self.wrong_guesses.contains(&guess) {
            return Guess::AlreadyGuessed;
        }
        match self.current_word.contains(guess) {
            true => Guess::Right,
            false => Guess::Wrong,
        }
    }

    fn results_so_far(&self) -> String {
        let mut output = String::new();
        for c in self.current_word.chars() {
            if self.right_guesses.contains(&c) {
                output.push(c)
            } else {
                output.push('*')
            }
        }
        output
        // let output = self
        //     .current_word
        //     .chars()
        //     .map(|c| {
        //         if self.right_guesses.contains(&c) {
        //             c
        //         } else {
        //             '*'
        //         }
        //     })
        //     .collect::<String>();
        // println!("{output}");
    }

    fn take_guess(&mut self, guess: String) -> String {
        let guess = guess.to_lowercase();
        let mut output = String::new();
        match guess {
            guess if guess.chars().count() == 1 => {
                let the_guess = guess.chars().next().unwrap();

                match self.check_guess(the_guess) {
                    Guess::AlreadyGuessed => {
                        output.push_str(&format!("You already guessed {the_guess}!\n"));
                    }
                    Guess::Right => {
                        self.right_guesses.push(the_guess);
                        output.push_str(&format!("Yes, it contains a {the_guess}!\n"));
                    }
                    Guess::Wrong => {
                        self.wrong_guesses.push(the_guess);
                        output.push_str(&format!("Nope, it doesn't contain a {the_guess}!\n"));
                    }
                }
                output.push_str(&self.results_so_far());
            }

            guess => {
                if self.current_word == guess {
                    output.push_str(&format!("You guessed right, it's {}! Let's play again!", self.current_word));
                } else {
                     output.push_str(&format!("Bzzt! it's not '{guess}', it's {}.\nTime to move on to another word!", self.current_word));
                }
                self.start();
            }
        }
        output
    }
}

async fn double(Path(input): Path<String>) -> String {
    match input.parse::<i32>() {
        Ok(num) => format!("{} times 2 is {}", num, num * 2),
        Err(e) => format!("Uh oh, weird input: {e}")
    }
}

async fn get_res_from_static(Path(guess): Path<String>) -> String {
    GAME.lock().unwrap().take_guess(guess)
}

#[tokio::main]
async fn main() {
    let mut app = GameApp::default();
    app.start();

   let app = axum::Router::new()
       .route("/", get(|| async { "The server is running well!" }))
       .route("/game/:guess", get(get_res_from_static))
       .route("/double/:number", get(double));

    let socket = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    let tcp = TcpListener::bind(socket).await.unwrap();

    axum::serve(tcp, app.into_make_service())
        .await
        .expect("unable to start server");
}