use lindera_core::mode::Mode;
use lindera_dictionary::{DictionaryConfig, DictionaryKind};
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};
use regex::Regex;

pub fn preprocess(data: Vec<String>) -> Vec<String> {
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

  data
    .into_iter()
    .map(|x| {
      let url_replaced = url.replace_all(&x, "");
      let replaced = mention.replace_all(&url_replaced, "");
      let tokens = tokenizer.tokenize(&replaced).unwrap();

      tokens
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<&str>>()
        .join(" ")
    })
    .collect::<Vec<String>>()
}