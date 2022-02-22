#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod decode;
mod encode;
mod types;

pub use decode::{Decodable, DecodeError};
pub use encode::{
    const_add, encode_fixed_size, encode_list, length_of_length, list_length, Encodable,
    MaxEncodedLen, MaxEncodedLenAssoc,
};
pub use types::Header;

#[cfg(feature = "alloc")]
pub use decode::decode_list;

#[cfg(feature = "derive")]
pub use fastrlp_derive::{RlpDecodable, RlpEncodable, RlpMaxEncodedLen};
