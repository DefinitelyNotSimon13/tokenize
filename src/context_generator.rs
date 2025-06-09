use std::{
    fs::File,
    io::{self, BufWriter, Write},
    sync::{Arc, Mutex},
};

use ignore::{WalkParallel, WalkState};
use log::warn;

use crate::{config::Config, file_data::FileData};

pub struct ContextGenerator<'a> {
    config: &'a Config,
    writer: Arc<Mutex<BufWriter<File>>>,
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

        Ok(ContextGenerator { config, writer })
    }
}

impl ContextGenerator<'_> {
    pub fn generate(&mut self, walker: WalkParallel) -> io::Result<()> {
        let default_prompt = include_bytes!("../assets/initial_prompt.md");
        // let initial_prompt = fs::read(self.config.prompt_file.clone())?;
        {
            let mut w = self.writer.lock().unwrap();
            w.write_all(default_prompt)?;
        }

        let writer = Arc::clone(&self.writer);
        let out_filename = self.config.out_file.file_name();
        walker.run(|| {
            // This closure runs once per thread, returning the actual visitor
            let writer = Arc::clone(&writer);
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
                    let fd = FileData::read(&path)?;
                    fd.write(&mut *writer.lock().unwrap())?;
                    Ok(())
                })() {
                    warn!("{}: {e}", path.display());
                }

                WalkState::Continue
            })
        });

        Ok(())
    }
}
