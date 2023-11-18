use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use lindera_core::mode::Mode;
use lindera_dictionary::{DictionaryConfig, DictionaryKind};
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};
use markov::Chain;
use once_cell::sync::Lazy;
use regex::*;
use serde::Deserialize;
use serde_json;
use tokio::sync::RwLock;

#[derive(Deserialize)]
struct Query {
  token: Option<String>,
}

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
  let url = Regex::new(r#"/(?:[a-zA-Z][a-zA-Z0-9+.-]*:)(?://(?:([^:@/?#\[\]\s]+)(?::([^:@/?#\[\]\s]*))?@)?(?:\[(?:[0-9A-Fa-f]{1,4}:){6}|(?:(?:[0-9A-Fa-f]:){0,5}[0-9A-Fa-f]?::(?:[0-9A-Fa-f]:){0,4}[0-9A-Fa-f]{1,4}$)(?:[0-9A-Fa-f]{1,4}:){1,5}|::(?:[0-9A-Fa-f]{1,4}:){0,5}[0-9A-Fa-f]{1,4}:)?(?:[0-9A-Fa-f]{1,4}:)?(?:(?:25[0-5]|(?:2[0-4]|1\d|[1-9])?\d)(?:\.(?:25[0-5]|(?:2[0-4]|1\d|[1-9])?\d)){3}|(?:[0-9A-Fa-f]{1,4}:){0,4}(?:(?:25[0-5]|(?:2[0-4]|1\d|[1-9])?\d)(?:\.(?:25[0-5]|(?:2[0-4]|1\d|[1-9])?\d)){3})?|(?:(?:[0-9A-Fa-f]{1,4}:)?(?:25[0-5]|(?:2[0-4]|1\d|[1-9])?\d))(?:\.(?:25[0-5]|(?:2[0-4]|1\d|[1-9])?\d)){3}))\]|(?:[^:@/?#\[\]\s]+)(?::(\d*))?(/(?:[^\s?#]*))?(?:\?([^#]*))?(?:#(.*))?/"#).unwrap();
  let mention = Regex::new(r#"(<a?(:[a-zA-Z_0-9]+:|(@|#)(!|&)?)\d+>)"#).unwrap();

  for message in messages {
    let url_replaced = url.replace_all(&message, "");
    let replaced = mention.replace_all(&url_replaced, "");
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
async fn generate_sentence(query: web::Query<Query>) -> impl Responder {
  let lock = CHAIN.read().await;

  match &query.token {
    Some(t) => lock.generate_str_from_token(&t),
    None => lock.generate_str(),
  }
}
