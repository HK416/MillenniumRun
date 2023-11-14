use ab_glyph::FontArc;

use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{
        AppResult,
        GameError,
    },
};



/// #### 한국어 </br>
/// 폰트 에셋의 디코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a decoder for font asset. </br>
/// 
#[derive(Debug)]
pub struct FontDecoder;

impl AssetDecoder for FontDecoder {
    type Output = FontArc;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        FontArc::try_from_vec(buf.to_vec())
            .map_err(|err| game_err!(
                "Failed to load font asset",
                "Loading the font asset failed for the following reasons: {}",
                err.to_string()
            ))
    }
}
