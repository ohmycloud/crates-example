use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
    verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Calculation {
    Perimeter,
    Area,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "shape")]
enum Shape {
    Circle { radius: f64 },
    Rectangle { length: f64, width: f64 },
}

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    calculation: Calculation,
    #[serde(flatten)]
    shape: Shape,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    result: f64,
}

fn parse_person(json_str: &str) -> anyhow::Result<Person> {
    Ok(serde_json::from_str::<Person>(json_str)?)
}

fn parse_request(json_str: &str) -> anyhow::Result<Request> {
    Ok(serde_json::from_str::<Request>(json_str)?)
}

fn calculation_handler(request: Request) -> Response {
    let result = match (request.calculation, request.shape) {
        (Calculation::Perimeter, Shape::Circle { radius }) => PI * 2.0 * radius,
        (Calculation::Perimeter, Shape::Rectangle { length, width }) => (length + width) * 2.0,
        (Calculation::Area, Shape::Circle { radius }) => PI * radius * radius,
        (Calculation::Area, Shape::Rectangle { length, width }) => length * width,
    };
    Response { result }
}

// https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/
#[tokio::main]
async fn main() {
    let json_str = r#"
        {
            "name": "George",
            "age": 27,
            "verified": false,
            "yada": "Yup"
        }
    "#;
    let person = parse_person(json_str);
    if let Ok(person) = person {
        println!("{:?}", person);
    }

    let json_str = r#"
        {
            "calculation": "perimeter",
            "shape": "circle",
            "radius": 2.3
        }
    "#;
    let request = parse_request(json_str);
    if let Ok(request) = request {
        println!("{:?}", request);
    }

    let endpoint = warp::post()
        .and(warp::body::json())
        .map(|body| warp::reply::json(&calculation_handler(body)));
    warp::serve(endpoint).run(([127, 0, 0, 1], 5000)).await;
}
