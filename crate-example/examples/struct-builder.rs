use bon::Builder;

#[derive(Builder, Debug)]
struct User {
    name: String,
    is_admin: bool,
    level: Option<u32>,
}

fn main() {
    let user = User::builder()
        .name("Bon".to_owned())
        .level(24)
        .is_admin(true)
        .build();
    println!("{:?}", user);
}
