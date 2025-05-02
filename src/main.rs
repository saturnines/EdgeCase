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
    let random_json 
}