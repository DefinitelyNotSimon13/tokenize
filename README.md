# Tokenize

Tokenize your codebase blazingly fast into a single file for LLM Context 🚀🤖

This tool uses [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) to actually tokenize files and provide accurate token counts for different OpenAI encoding models.

## Usage

**Basic Usage**
```sh
tokenize [TARGET_DIR]
```

**Show token counts for each file and total**
```sh
tokenize [TARGET_DIR] --show-tokens
```

**Specify a custom output file**
```sh
tokenize . -o output.md
```

**Choose a different encoding model**
```sh
tokenize . --model cl100k-base --show-tokens
```

Available models:
- `o200k-base` (default) - GPT-5, GPT-4.1, GPT-4o, o4, o3, and o1 models
- `cl100k-base` - ChatGPT models, text-embedding-ada-002
- `p50k-base` - Code models, text-davinci-002, text-davinci-003
- `p50k-edit` - Edit models like text-davinci-edit-001
- `r50k-base` - GPT-3 models like davinci

**[INOP] Specify a custom prompt for the beginning of the file**
```sh
tokenize . -p prompt.txt
```

**Don't respect .gitignore files**
```sh
tokenize [TARGET_DIR] --no-gitignore
```

**Include hidden files**
```sh
tokenize [TARGET_DIR] --include-hidden
```

**Follow Symlinks**
_This was not tested, I am not sure if it actually includes the files in the final output_
```sh
tokenize [TARGET_DIR] --follow-symlinks
```

## Attribution
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore)
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)
