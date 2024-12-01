use bon::builder;

#[builder]
fn greet(language: &str, level: Option<u32>) -> String {
    let level = level.unwrap_or(0);
    format!("Hello, {language} {level}")
}

fn main() {
    let greeting = greet().language("Perl").level(6).call();
    println!("{}", greeting);
}
