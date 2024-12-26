use anyhow::{anyhow, Context, Error};

fn send_number(number: i32) -> Result<(), Error> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        println!("Too large!");
        Err(anyhow!("Number is too large"))
    }
}

fn parse_then_send(input: &[u8]) -> Result<(), Error> {
    let some_str = std::str::from_utf8(input)
        .with_context(|| format!("{input:?} 无法被解析成 str 字符串"))?;
    let number = some_str.parse::<i32>()
        .with_context(|| format!("这个字符串有点奇怪: {some_str}, 我没法把它解析成数字"))?;
    send_number(number)?;
    Ok(())
}

fn main() {
    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"42"));
}