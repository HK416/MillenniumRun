use serde::{Serialize, Deserialize};

use crate::{
    game_err, 
    assets::interface::{AssetDecoder, AssetEncoder},
    system::error::{AppResult, GameError}, 
};



/// #### 한국어 </br>
/// 게임 스테이지의 클리어 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains clear on of the game stage. </br> 
/// 
#[repr(C)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveData {
    pub stage_aris: u16, 
    pub stage_momoi: u16, 
    pub stage_midori: u16, 
    pub stage_yuzu: u16, 
    pub beginner: bool, 
}

impl Default for SaveData {
    #[inline]
    fn default() -> Self {
        Self {
            stage_aris: 0, 
            stage_momoi: 0, 
            stage_midori: 0, 
            stage_yuzu: 0, 
            beginner: true
        }
    }
}



/// #### 한국어 </br>
/// 세이브 데이터의 디코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a decoder for save data. </br>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveDecoder;

impl AssetDecoder for SaveDecoder {
    type Output = SaveData;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        let output: SaveData = bincode::deserialize(buf)
            .map_err(|err| game_err!(
                "Failed to load save file", 
                "The save file failed to load for the following reasons: {}", 
                err.to_string()
            ))?;

        is_validate(output.stage_aris)?;
        is_validate(output.stage_momoi)?;
        is_validate(output.stage_midori)?;
        is_validate(output.stage_yuzu)?;

        return Ok(output);
    }
}

#[inline]
fn is_validate(num_owned_tiles: u16) -> AppResult<()> {
    use crate::nodes::in_game::NUM_TILES;
    if num_owned_tiles > NUM_TILES as u16 {
        return Err(game_err!("Failed to load asset file", "Corrupted save data."));
    }
    return Ok(())
}



/// #### 한국어 </br>
/// 세이브 데이터의 인코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a encoder for save data. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveEncoder;

impl AssetEncoder for SaveEncoder {
    type Input = SaveData;

    #[inline]
    fn encode(&self, val: &Self::Input) -> AppResult<Vec<u8>> {
        let byte = bincode::serialize(val)
            .map_err(|err| game_err!(
                "Failed to store save file", 
                "The save file failed to store for the following reasons: {}", 
                err.to_string()
            ))?;

        return Ok(byte);
    }
}
