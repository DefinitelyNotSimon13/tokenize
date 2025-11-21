use std::{
    fs::File,
    io::{self, BufWriter, Write},
    sync::{Arc, Mutex},
};

use ignore::{WalkParallel, WalkState};
use log::warn;

use crate::{config::Config, file_data::FileData, tokenizer::Tokenizer};

pub struct ContextGenerator<'a> {
    config: &'a Config,
    writer: Arc<Mutex<BufWriter<File>>>,
    tokenizer: Arc<Tokenizer>,
    total_tokens: Arc<Mutex<usize>>,
}

impl<'a> ContextGenerator<'a> {
    pub fn new(config: &'a Config) -> io::Result<Self> {
        if config.out_file.exists() {
            warn!(
                "out file {} already exists and will be overwritten",
                config.out_file.display()
            );
        }

        let out_file = File::create(config.out_file.clone())?;
        let writer = Arc::new(Mutex::new(BufWriter::new(out_file)));
        
        let tokenizer = Tokenizer::new(config.model).map_err(|e| {
            io::Error::other(format!("Failed to create tokenizer: {e}"))
        })?;

        Ok(ContextGenerator {
            config,
            writer,
            tokenizer: Arc::new(tokenizer),
            total_tokens: Arc::new(Mutex::new(0)),
        })
    }
}

impl ContextGenerator<'_> {
    pub fn generate(&mut self, walker: WalkParallel) -> io::Result<()> {
        let default_prompt = include_bytes!("../assets/initial_prompt.md");
        // let initial_prompt = fs::read(self.config.prompt_file.clone())?;
        
        // Write initial prompt and optionally count its tokens
        {
            let mut w = self.writer.lock().unwrap();
            w.write_all(default_prompt)?;
        }
        
        if self.config.show_tokens
            && let Ok(prompt_text) = String::from_utf8(default_prompt.to_vec())
        {
            let prompt_tokens = self.tokenizer.count_tokens(&prompt_text);
            let mut total = self.total_tokens.lock().unwrap();
            *total += prompt_tokens;
        }

        let writer = Arc::clone(&self.writer);
        let tokenizer = Arc::clone(&self.tokenizer);
        let total_tokens = Arc::clone(&self.total_tokens);
        let show_tokens = self.config.show_tokens;
        let out_filename = self.config.out_file.file_name();
        
        walker.run(|| {
            // This closure runs once per thread, returning the actual visitor
            let writer = Arc::clone(&writer);
            let tokenizer = Arc::clone(&tokenizer);
            let total_tokens = Arc::clone(&total_tokens);
            
            Box::new(move |res| {
                if let Err(err) = &res {
                    warn!("Walk error: {err}");
                    return WalkState::Continue;
                }
                let entry = res.unwrap();

                if !entry.file_type().is_some_and(|ft| ft.is_file()) {
                    return WalkState::Continue;
                }
                let path = entry.into_path();

                if path.file_name() == out_filename {
                    return WalkState::Continue;
                }

                if let Err(e) = (|| -> io::Result<()> {
                    let mut fd = FileData::read(&path)?;
                    
                    if show_tokens {
                        fd.tokenize(&tokenizer);
                        if let Some(count) = fd.token_count() {
                            let mut total = total_tokens.lock().unwrap();
                            *total += count;
                        }
                    }
                    
                    fd.write(&mut *writer.lock().unwrap(), show_tokens)?;
                    Ok(())
                })() {
                    warn!("{}: {e}", path.display());
                }

                WalkState::Continue
            })
        });

        // Write total token count if enabled
        if self.config.show_tokens {
            let total = *self.total_tokens.lock().unwrap();
            writeln!(self.writer.lock().unwrap(), "\n-------- Total Tokens: {total} --------")?;
        }

        Ok(())
    }
}
