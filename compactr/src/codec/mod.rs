//! Encoding and decoding functionality.

pub mod buffer;
mod decoder;
mod encoder;
mod traits;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use traits::{Decode, Encode};
