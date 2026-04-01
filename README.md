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

# Generate an array with strings in JSON format
texto text -f json -c 2

# Generate words with a separator
texto text -s ","

# Generate Lorem Ipsum
texto lorem

# Generate Lorem Ipsum with 10 words
texto lorem -w 10

# Generate a UUID (Default: Version 6)
texto uuid

# Generate a v7 UUID
texto uuid -v v7

# Save 10 UUIDs to uuids.txt
texto uuid -c 10 -o "uuids.txt"

# Generate a UUID with hyphen
texto uuid -f hyphenated
```

## License
MIT License - [LICENSE](LICENSE)
