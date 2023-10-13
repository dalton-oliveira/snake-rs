use bincode::{
    config, de, decode_from_slice, enc, encode_to_vec,
    error::{self, EncodeError},
};

pub fn encode<E: enc::Encode>(val: E) -> Result<Vec<u8>, EncodeError> {
    encode_to_vec(val, config::standard())
}

pub fn decode<D: de::Decode>(src: &[u8]) -> Result<(D, usize), error::DecodeError> {
    decode_from_slice(src, config::standard())
}
