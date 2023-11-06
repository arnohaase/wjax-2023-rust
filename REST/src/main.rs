use std::net::SocketAddr;
use std::str::FromStr;
use axum::{Json, Router, Server, ServiceExt};
use axum::extract::Path;
use axum::routing::get;
use serde::{Deserialize, Serialize};


fn main_json() -> anyhow::Result<()> {
    let person = Person {
        id: None,
        name: "whoever".to_string(),
        age: 123,
    };

    let mut w = String::new();
    let s = serde_json::to_string(&person)?;

    let deser_person: Person = serde_json::from_str(&s)?;
    println!("serialized: {}, {:#?}", s, deser_person);

    Ok(())
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
    ;

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Halli hallo"
}



#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: Option<u64>,
    name: String,
    age: u8,
}
