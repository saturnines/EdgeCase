use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::{env, fs};
use dotenv::dotenv;
use tokio::net::TcpListener;
use serde_json::Value;
use std::sync::Arc;
use once_cell::sync::Lazy;
use rand::seq::IndexedRandom;

// Joke Structure
#[derive(Deserialize, Serialize, Clone)]
struct Joke {
    id: usize,
    contributor: String,
    joke: String,
    punchline: String
}

// Load JSON
static JOKES: Lazy<Arc<Vec<Joke>>> = Lazy::new(|| {
    let json_content = fs::read_to_string("jokes.json")
        .expect("Could not read jokes.json or jokes.json doesn't exist!");
    let jokes: Vec<Joke> = serde_json::from_str(&json_content)
        .expect("JSON was not formatted correctly");
    Arc::new(jokes)
});


async fn joke() -> Json<Value> {
    let jokes = &*JOKES;

    let joke = jokes
        .choose(&mut rand::rng()) // Use thread_rng() instead of rng()
        .cloned()
        .unwrap_or_else(|| Joke {
            id: 0,
            contributor: "".to_string(),
            joke: "No jokes currently".to_string(),
            punchline: "".to_string(),
        });

    let response = serde_json::json!({
        "id": joke.id,
        "joke": joke.joke
    });

    Json(response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    
    let _ = &*JOKES;
    println!("Loaded {} jokes", JOKES.len());

    let app = Router::new()
        .route("/joke", get(joke));

    // Host and Port
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = format!("{}:{}", host, port);
    println!("Server running on http://{}", addr);

    // Create a TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    // Use the new serve function
    axum::serve(listener, app).await.unwrap();
}
//