use crate::{
    app::abort::AppResult,
    assets::interface::AssetDecoder,
};



#[derive(Debug)]
pub struct ShaderDecoder;

impl AssetDecoder for ShaderDecoder {
    type Output = String;

    #[inline]
    fn decode(buf: &[u8]) -> AppResult<Self::Output> {
        Ok(String::from_utf8_lossy(buf).into_owned())
    }
}
