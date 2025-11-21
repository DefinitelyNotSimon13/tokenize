use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

/// ASCII space character - characters below this (except whitespace) are considered non-printable
const ASCII_SPACE: u8 = 0x20;

/// Maximum ratio of non-printable characters before considering a file binary
const BINARY_THRESHOLD: f64 = 0.3;

/// Sample size (in bytes) to check for binary content detection
const BINARY_SAMPLE_SIZE: usize = 8192;

/// Common binary file extensions that should be skipped
const BINARY_EXTENSIONS: &[&str] = &[
    // Images (excluding SVG which is XML/text)
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "tiff", "tif",
    // Videos
    "mp4", "avi", "mov", "mkv", "wmv", "flv", "webm",
    // Audio
    "mp3", "wav", "ogg", "flac", "aac", "m4a",
    // Archives
    "zip", "tar", "gz", "bz2", "7z", "rar", "xz", "zst",
    // Executables and libraries
    "exe", "dll", "so", "dylib", "bin", "o", "obj", "a", "lib",
    // Fonts
    "ttf", "otf", "woff", "woff2", "eot",
    // Documents (binary formats)
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
    // Databases
    "db", "sqlite", "sqlite3",
    // Other common binary formats
    "pyc", "class", "jar", "war", "wasm",
];

/// Common text file extensions that can skip binary detection
const TEXT_EXTENSIONS: &[&str] = &[
    // Programming languages
    "rs", "go", "py", "js", "ts", "jsx", "tsx", "java", "c", "cpp", "cc", "h", "hpp",
    "cs", "php", "rb", "swift", "kt", "scala", "sh", "bash", "zsh", "fish",
    // Web
    "html", "htm", "css", "scss", "sass", "less", "vue", "svg",
    // Data/Config
    "json", "xml", "yaml", "yml", "toml", "ini", "conf", "cfg",
    // Markup/Documentation
    "md", "markdown", "rst", "txt", "text", "log",
    // Other
    "sql", "graphql", "proto", "dockerfile",
];

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

    /// Checks if a file should be skipped based on extension alone (performance optimization).
    /// Returns true if the file is definitely binary based on extension.
    #[must_use]
    pub fn is_binary_by_extension(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            return BINARY_EXTENSIONS.contains(&ext_lower.as_str());
        }
        false
    }

    /// Checks if a file is definitely text based on extension (performance optimization).
    /// Returns true if the file is definitely text and can skip content-based detection.
    #[must_use]
    pub fn is_text_by_extension(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            return TEXT_EXTENSIONS.contains(&ext_lower.as_str());
        }
        false
    }

    /// Efficiently checks if a file is binary by reading only a sample.
    /// Returns true if the file appears to be binary based on content analysis.
    #[must_use]
    pub fn is_binary_by_content(path: &Path) -> bool {
        let Ok(mut file) = File::open(path) else {
            return false; // If we can't open it, treat as text to avoid skipping
        };

        let mut buffer = vec![0u8; BINARY_SAMPLE_SIZE];
        let Ok(bytes_read) = file.read(&mut buffer) else {
            return false;
        };

        if bytes_read == 0 {
            return false; // Empty files are text
        }

        let sample = &buffer[..bytes_read];

        // Fast check: if file contains null bytes, it's likely binary
        if sample.contains(&0) {
            return true;
        }

        // Check for ratio of non-printable characters
        let non_text_count = sample
            .iter()
            .filter(|&&b| b < ASCII_SPACE && b != b'\n' && b != b'\r' && b != b'\t')
            .count();

        // Precision loss in f64 conversion is acceptable for ratio comparison
        #[allow(clippy::cast_precision_loss)]
        {
            non_text_count as f64 / sample.len() as f64 > BINARY_THRESHOLD
        }
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
    /// Note: This method assumes the file content is already loaded. For performance-critical
    /// paths, use `is_binary_by_extension()` and `is_binary_by_content()` instead.
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
        // Precision loss in f64 conversion is acceptable for ratio comparison
        #[allow(clippy::cast_precision_loss)]
        {
            non_text_count as f64 / sample.len() as f64 > BINARY_THRESHOLD
        }
    }
}
