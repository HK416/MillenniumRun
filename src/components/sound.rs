use std::io::Cursor;

use rodio::{Sink, OutputStreamHandle};

use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{AppResult, GameError},
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

#[inline]
pub fn create_sink(stream_handle: &OutputStreamHandle) -> AppResult<Sink> {
    Sink::try_new(stream_handle)
        .map_err(|err| game_err!(
            "Sound player creation failed",
            "Sound player creation failed for following reasons: {}",
            err.to_string()
        ))
}
