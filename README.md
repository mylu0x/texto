# Texto
**Texto** is a CLI for generating dummy text.

> [!IMPORTANT]
> This project is still under development. As a Rust begineer's project, you may encounter bugs or unexpected behavior.

## Installation

### Using Cargo (Recommend)
```bash
cargo install texto
```

### Direct Download
Download the latest binary from [Releases](https://github.com/mylu0x/texto/releases) and add it to your PATH.

## Usage
```bash
# Generate words (16 words, English) 
texto text

# Generate 100 words
texto text -w 100

# Generate 10 words in German
texto text -l de

# Generate 100 words three times
texto text -w 100 -c 3
```

## License
MIT License - [LICENSE](LICENSE)
