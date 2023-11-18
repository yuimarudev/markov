use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

use markov::Chain;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::RwLock;
use gen_markov::preprocess;

#[derive(Deserialize)]
struct Query {
  token: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PreprocessedMessages {
  flag: i32,
  data: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Raw(Vec<String>);

static CHAIN: Lazy<RwLock<Chain<String>>> = Lazy::new(|| RwLock::new(Chain::new()));

#[tokio::main]
async fn main() {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let fname = std::env::var("file").unwrap_or("main.json".to_string());
  let data = std::fs::read_to_string(&fname).unwrap();
  let messages = match serde_json::from_str::<Raw>(&data) {
    Ok(f) => preprocess(f.0),
    Err(_) => serde_json::from_str::<PreprocessedMessages>(&data).unwrap().data
  };

  for message in messages {
    let mut lock = CHAIN.write().await;

    lock.feed_str(&message);

    drop(lock);
  }

  HttpServer::new(|| {
    App::new()
      .service(generate_sentence)
      .wrap(Logger::default())
  })
  .bind(("0.0.0.0", 8931))
  .unwrap()
  .run()
  .await
  .unwrap();
}


#[get("/")]
async fn generate_sentence(query: web::Query<Query>) -> impl Responder {
  let lock = CHAIN.read().await;

  match &query.token {
    Some(t) => lock.generate_str_from_token(&t),
    None => lock.generate_str(),
  }
}
