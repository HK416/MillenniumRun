use std::io::Cursor;

use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{
        AppResult,
        GameError,
    },
};

#[derive(Debug)]
pub struct SoundDecoder;

impl AssetDecoder for SoundDecoder {
    type Output = rodio::Decoder<Cursor<Vec<u8>>>;

    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        let cursor = Cursor::new(buf.to_vec());
        rodio::Decoder::new(cursor)
            .map_err(|err| game_err!(
                "Sound decoding failed",
                "Sound decoding failed for following reasons: {}",
                err.to_string()
            ))
    }
}
