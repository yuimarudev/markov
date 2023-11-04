use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer, Responder};
use lindera_core::mode::Mode;
use lindera_dictionary::{DictionaryConfig, DictionaryKind};
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};
use markov::Chain;
use once_cell::sync::Lazy;
use regex::*;
use serde_json;
use tokio::sync::RwLock;

static CHAIN: Lazy<RwLock<Chain<String>>> = Lazy::new(|| RwLock::new(Chain::new()));

#[tokio::main]
async fn main() {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let messages = serde_json::from_str::<Vec<String>>(include_str!("main.json")).unwrap();
  let dictionary = DictionaryConfig {
    kind: Some(DictionaryKind::IPADIC),
    path: None,
  };
  let tokenizer_config = TokenizerConfig {
    dictionary,
    user_dictionary: None,
    mode: Mode::Normal,
  };
  let tokenizer = Tokenizer::from_config(tokenizer_config).unwrap();
  let re = Regex::new(r#"<(a?:[a-zA-Z\-]+:\d{17,20})|((@|&)!?\d{17,20})>"#).unwrap();

  for message in messages {
    let replaced = re.replace_all(&message, "");
    let tokens = tokenizer.tokenize(&replaced).unwrap();

    let mut lock = CHAIN.write().await;

    lock.feed_str(
      &tokens
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<&str>>()
        .join(" "),
    );

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
async fn generate_sentence() -> impl Responder {
  let lock = CHAIN.read().await;

  lock.generate_str()
}
