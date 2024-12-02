//! Royal is a library for parsing game dialogue message formats.
//! 
//! # Example
//! ```
//! use royal::Message;
//! 
//! let input = "[msg MSG_BTTL_2 [Morgana]][s][f 4 10 65535 0 0]Hello world![f 1 3 65535][w][e]";
//! if let Some(message) = Message::parse(input) {
//!     assert_eq!(message.content, "Hello world!");
//!     assert!(message.flags.has_lipsync);
//! }
//! ```

mod parser;

pub use parser::*;