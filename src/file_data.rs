use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct FileData {
    path: PathBuf,
    content: Vec<u8>,
}

impl FileData {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let path = path.as_ref();
        let content = fs::read(path)?;

        Ok(Self {
            path: path.to_path_buf(),
            content,
        })
    }

    pub fn write<W: Write>(&self, buf: &mut W) -> io::Result<()> {
        writeln!(
            buf,
            "-------- {} --------\n```{}",
            self.path.display(),
            self.extension()
        )?;
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
}
