#![warn(unused_doc_comments)]

use serde::de::{Error, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq, Serialize)]
struct MyStruct {
    message: String,
}

fn to_and_from_json() {
    let json_str = r#"{"message": "Hello world!"}"#;
    let my_struct: MyStruct = serde_json::from_str(&json_str).unwrap();
    assert_eq!(
        my_struct,
        MyStruct {
            message: "Hello world!".into()
        }
    );
    assert!(serde_json::to_string(&my_struct).is_ok())
}

// 手动实现序列化和反序列化
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("A message that can either be deserialized from an i32 or String")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_owned())
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for MyStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // note: don't use unwrap in production!
        let message = deserializer.deserialize_string(MessageVisitor).unwrap();
        Ok(Self { message })
    }
}

fn main() {
    to_and_from_json();
    // println!("Test started...");
    //for_in_iterator();
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    /// 注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    /// 换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn for_in_iterator() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    /// 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
    for element in v1_iter {
        println!("Got: {}", element);
    }
}

#[test]
fn consumer_iter() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
