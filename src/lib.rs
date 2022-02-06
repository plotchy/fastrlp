#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod decode;
mod encode;
mod types;

pub use decode::{Decodable, DecodeError};
pub use encode::{encode_list, list_length, Encodable};
pub use types::Header;

#[cfg(feature = "alloc")]
pub use decode::decode_list;
