use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{
        AppResult, 
        GameError
    }, 
};



/// #### 한국어 </br>
/// 스크립트의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of tags in the script. </br>
/// 
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScriptTags {
    NotifyTitle,
    NotifyText,
    StartMenu,
    SettingMenu,
    ExitMenu,
    Exit,
    NoExit,
    Store,
    NoStore,
    ExitMessage,
    EnterStage,
}



/// #### 한국어 </br>
/// 사용자가 선택한 언어에 대한 스크립트를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the script for the language selected by the user. </br>
/// 
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script(HashMap<ScriptTags, String>);

impl Script {
    /// #### 한국어 </br>
    /// 태그에 해당하는 스크립트를 가져옵니다. </br>
    /// 이때, 해당 스크립트가 존재하지 않을 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Get the script corresponding to the tag. </br>
    /// At this time, if the script does not exist, `GameError` is returned. </br>
    /// 
    #[inline]
    pub fn get(&self, tag: ScriptTags) -> AppResult<&String> {
        self.0.get(&tag).ok_or_else(|| {
            game_err!("Game Logic Error", "This is an unspecified script.")
        })
    }
}



/// #### 한국어 </br>
/// `ron` 형식으로 작성된 스크립트를 읽는 디코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a decoder that reads scripts written in `ron` format. </br>
/// 
#[derive(Debug)]
pub struct ScriptDecoder;

impl AssetDecoder for ScriptDecoder {
    type Output = Script;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        ron::de::from_bytes(buf)
            .map_err(|err| game_err!(
                "Script decoding failed",
                "Script decoding failed for the following reasons: {}",
                err.to_string()
            ))
    }
}
