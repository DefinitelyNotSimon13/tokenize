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

    /// Checks if a file is binary using a three-tier detection system.
    /// 
    /// This function uses fast paths to avoid unnecessary I/O:
    /// 1. Extension-based detection for known binary formats (instant, no I/O)
    /// 2. Extension-based detection for known text formats (instant, no I/O)
    /// 3. Content-based detection for unknown extensions (reads only first 8KB)
    /// 
    /// Returns true if the file is binary, false if it's text.
    #[must_use]
    pub fn is_binary_file(path: &Path) -> bool {
        // Fast path 1: Check if it's a known binary extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            
            // Known binary extension - skip immediately
            if BINARY_EXTENSIONS.contains(&ext_lower.as_str()) {
                return true;
            }
            
            // Known text extension - skip content check
            if TEXT_EXTENSIONS.contains(&ext_lower.as_str()) {
                return false;
            }
        }

        // Fallback: Unknown extension - check content by reading only a sample
        // This is the slowest path but only used for files with unknown extensions
        Self::is_binary_by_content_check(path)
    }

    /// Internal helper: checks if file content is binary by reading only a sample.
    /// Only called for files with unknown extensions.
    fn is_binary_by_content_check(path: &Path) -> bool {
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
}
