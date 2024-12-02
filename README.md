# Royal

A Rust library for parsing game dialogue message formats.

## Features

- Parse structured dialogue messages
- Support for character names, message IDs, and box types
- Handle lipsync flags and wait-for-input markers
- Clean content extraction

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
royal = "0.1.2"
```

Example usage:

```rust
use royal::Message;

fn main() {
    let input = "[msg MSG_BTTL_2 [Morgana]][s][f 4 10 65535 0 0]Hello world![f 1 3 65535][w][e]";
    
    if let Some(message) = Message::parse(input) {
        println!("Character: {}", message.header.character.unwrap_or_default());
        println!("Content: {}", message.content);
    }
}
```

## Message Format

The parser handles messages in the following format:
- Header: `[msg MESSAGE_TYPE_MESSAGE_ID [CHARACTER]]`
- Content markers:
  - `[s]`: Start of message
  - `[f 4 10 65535 0 0]`: Lipsync marker
  - `[f 1 3 65535]`: Message end marker
  - `[w]`: Wait for input
  - `[e]`: End of message

## License

This project is licensed under the GNU General Public License v3.0 or later - see the [LICENSE](LICENSE) file for details.