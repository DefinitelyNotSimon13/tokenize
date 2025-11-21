use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::tokenizer::Tokenizer;

#[derive(Debug)]
pub struct FileData {
    path: PathBuf,
    content: Vec<u8>,
    token_count: Option<usize>,
}

impl FileData {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let path = path.as_ref();
        let content = fs::read(path)?;

        Ok(Self {
            path: path.to_path_buf(),
            content,
            token_count: None,
        })
    }

    pub fn tokenize(&mut self, tokenizer: &Tokenizer) {
        if let Ok(text) = std::str::from_utf8(&self.content) {
            self.token_count = Some(tokenizer.count_tokens(text));
        }
    }

    pub fn write<W: Write>(&self, buf: &mut W, show_tokens: bool) -> io::Result<()> {
        let header = if show_tokens {
            self.token_count.map_or_else(
                || format!("-------- {} --------", self.path.display()),
                |count| format!("-------- {} ({count} tokens) --------", self.path.display())
            )
        } else {
            format!("-------- {} --------", self.path.display())
        };
        
        writeln!(buf, "{header}\n```{}", self.extension())?;
        buf.write_all(&self.content)?;
        writeln!(buf, "```")
    }

    #[must_use]
    pub fn extension(&self) -> &str {
        self.path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    #[must_use]
    pub const fn token_count(&self) -> Option<usize> {
        self.token_count
    }
}
