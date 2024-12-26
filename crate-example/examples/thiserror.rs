use std::{num::ParseIntError, str::Utf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
enum SystemError {
    // this variant unrelated to any other external error types.
    // the zero in these attribute macros means .0 used when accessing a tuple
    #[error("Couldn't send: {0}")]
    SendError(String),
    // hold information from the Utf8Error in the standard library, so we will use #[from]
    #[error("不能被解析为 str 字符串: {0}")]
    StringFromUtf8Error(#[from] Utf8Error),
    #[error("不能被解析为 i32: {0}")]
    ParseI32Error(#[from] ParseIntError),
    #[error("不存在的颜色: Red {0}, Green {1}, Blue {2}")]
    ColorError(u8, u8, u8),
    #[error("发生了未知错误")]
    OtherError,
}

fn send_number(number: i32) -> Result<(), SystemError> {
    match number {
        num if num == 500 => Err(SystemError::OtherError),
        num if num == 42 => Err(SystemError::ColorError(1,2,3)),
        num if num > 1_000_000 => Err(SystemError::SendError(format!("{num} is too large, can't send!"))),
        _ => {
            println!("Number sent!");
            Ok(())
        }
    }
}

fn parse_then_send(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input)?;
    let number = some_str.parse::<i32>()?;
    send_number(number)?;
    Ok(())
}

fn main() {
    println!("{:?}", parse_then_send(b"nine").unwrap_err());
    println!("{:?}", parse_then_send(&[8, 9, 0, 200]).unwrap_err());
    println!("{:?}", parse_then_send(b"109080098").unwrap_err());
    println!("{:?}", parse_then_send(b"42").unwrap_err());
    parse_then_send(b"10098").unwrap();
}