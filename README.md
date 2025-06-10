# Tokenize

_Technically nothing is "tokenized", all files are simply put into one file along with some formatting and an initial prompt._

## Usage

*Basic Usage*
```sh
# (TARGET_DIR is currently not working, the current dir is always used)
tokenize [TARGET_DIR]
```

**Specify a custom output file**
```sh
tokenize . -o output.md
```

**[INOP] Specify a custom prompt for the beginning of the file**
```sh
tokenize . -p prompt.txt
```

**Dont respect .gitignore files**
```sh
tokenize [TARGET_DIR] --no-gitignore
```
_See [Issue #1](https://github.com/DefinitelyNotSimon13/tokenize/issues/1) regarding the target dir_

**Include hidden files**
```sh
tokenize [TARGET_DIR] --include-hidden
```

**Follow Symlinks**
_This was not tested, I am not sure if it actually includes the files in the final output_
```sh
tokenize [TARGET_DIR] --follow symlinks
```

## Attribution
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore)
