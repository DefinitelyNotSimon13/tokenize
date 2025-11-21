use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// ASCII space character - characters below this (except whitespace) are considered non-printable
const ASCII_SPACE: u8 = 0x20;

/// Maximum ratio of non-printable characters before considering a file binary
const BINARY_THRESHOLD: f64 = 0.3;

/// Sample size (in bytes) to check for binary content detection
const BINARY_SAMPLE_SIZE: usize = 8192;

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

    /// Checks if the file content appears to be binary.
    /// A file is considered binary if it contains null bytes or has a high ratio of non-text bytes.
    #[must_use]
    pub fn is_binary(&self) -> bool {
        // Empty files are considered text
        if self.content.is_empty() {
            return false;
        }

        // Check first 8KB of the file for null bytes (common binary indicator)
        let sample_size = self.content.len().min(BINARY_SAMPLE_SIZE);
        let sample = &self.content[..sample_size];

        // If file contains null bytes, it's likely binary
        if sample.contains(&0) {
            return true;
        }

        // Check for ratio of non-printable characters
        let non_text_count = sample
            .iter()
            .filter(|&&b| b < ASCII_SPACE && b != b'\n' && b != b'\r' && b != b'\t')
            .count();

        // If more than the threshold of characters are non-printable, consider it binary
        non_text_count as f64 / sample.len() as f64 > BINARY_THRESHOLD
    }
}
