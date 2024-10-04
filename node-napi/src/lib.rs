#![deny(clippy::all)]

use napi_derive::napi;
use tokenizers::tokenizer::Tokenizer as RustTokenizer;

#[napi]
pub struct Tokenizer {
  tokenizer: RustTokenizer,
}

#[napi]
impl Tokenizer {
  #[napi(constructor)]
  pub fn new(bytes: Vec<u8>) -> napi::Result<Self> {
    let tokenizer =
      RustTokenizer::from_bytes(bytes).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self {
      tokenizer: tokenizer,
    })
  }

  #[napi]
  pub fn encode(&self, input: String, add_special_tokens: bool) -> napi::Result<Vec<u32>> {
    let output = self
      .tokenizer
      .encode(input, add_special_tokens)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(output.get_ids().to_vec())
  }

  #[napi]
  pub fn decode(&self, ids: Vec<u32>, skip_special_tokens: bool) -> napi::Result<String> {
    let output = self
      .tokenizer
      .decode(&ids, skip_special_tokens)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(output)
  }

  #[napi]
  pub fn encode_fast(&self, input: String, add_special_tokens: bool) -> napi::Result<Vec<u32>> {
    let output = self
      .tokenizer
      .encode_fast(input, add_special_tokens)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(output.get_ids().to_vec())
  }
}
