use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, fs};


// Joke Structure
#[derive(Deserialize, Serialize, Clone)]


// Ideally Punchline should never be used but just in case.,
struct Joke {
    id:usize, 
    contributor: String,
    joke: String,
    punchline: String 
}

// More like get observation lol
async fn get_joke() -> Json<Vec<Joke>> {
    let json_content = fs::read_to_string("jokes.json")
        .expect("Could not read jokes.json or jokes.json doesn't exist!");
    
    // Parse the JSON into joke 
    let jokes: Vec<Joke> = serde_json::from_str(&json_content)
        .expect("JSON was not formatted correctly"); 
    
    Json(jokes)
}