use std::sync::Arc;
use tiktoken_rs::CoreBPE;

use crate::cli::EncodingModel;

#[derive(Debug, thiserror::Error)]
pub enum TokenizerError {
    #[error("Failed to initialize tokenizer: {0}")]
    InitError(String),
}

pub struct Tokenizer {
    bpe: Arc<CoreBPE>,
}

impl Tokenizer {
    pub fn new(model: EncodingModel) -> Result<Self, TokenizerError> {
        let bpe = match model {
            EncodingModel::O200kBase => tiktoken_rs::o200k_base(),
            EncodingModel::Cl100kBase => tiktoken_rs::cl100k_base(),
            EncodingModel::P50kBase => tiktoken_rs::p50k_base(),
            EncodingModel::P50kEdit => tiktoken_rs::p50k_edit(),
            EncodingModel::R50kBase => tiktoken_rs::r50k_base(),
        }
        .map_err(|e| TokenizerError::InitError(e.to_string()))?;

        Ok(Self {
            bpe: Arc::new(bpe),
        })
    }

    #[must_use]
    pub fn encode(&self, text: &str) -> Vec<u32> {
        self.bpe.encode_with_special_tokens(text)
    }

    #[must_use]
    pub fn count_tokens(&self, text: &str) -> usize {
        self.encode(text).len()
    }
}
