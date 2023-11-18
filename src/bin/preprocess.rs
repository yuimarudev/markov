use gen_markov::preprocess;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct PreprocessedMessages {
  flag: i32,
  data: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Raw(Vec<String>);

#[tokio::main]
async fn main() {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let fname = std::env::var("file").unwrap_or("main.json".to_string());
  let data = std::fs::read_to_string(&fname).unwrap();
  let messages = PreprocessedMessages {
    flag: 1,
    data: preprocess(serde_json::from_str::<Raw>(&data).unwrap().0),
  };

  std::fs::write(fname, serde_json::to_string(&messages).unwrap()).unwrap();
}
