use std::io::Cursor;

use serde::{Serialize, Deserialize};
use rodio::{
    Sink,
    OutputStream, 
    OutputStreamHandle, 
    StreamError, 
    PlayError, 
};

use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{AppResult, GameError}, 
};



/// #### 한국어 </br>
/// 소리의 크기 데이터를 담고있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Thisis a structure that contains sound volume data. </br>
/// 
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Volume(u8);

impl Volume {
    /// #### 한국어 </br>
    /// 새로운 볼륨을 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create a new volume. </br>
    /// 
    #[inline]
    pub const fn new(val : u8) -> Self {
        Self(val)
    }

    /// #### 한국어 </br>
    /// 볼륨을 새로운 값으로 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set the volume to the new value. </br>
    /// 
    #[inline]
    pub fn set(&mut self, val: u8) {
        self.0 = val
    }

    /// #### 한국어 </br>
    /// `0.0 ~ 2.55` 사이의 값으로 변환된 볼륨 값을 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Get the volume value converted to a value between `0.0 and 2.55`. </br>
    /// 
    #[inline]
    pub fn norm(&self) -> f32 {
        self.0 as f32 / 100.0
    }
}




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


/// #### 한국어 </br>
/// 사용자의 기본 소리 출력 장치를 가져옵니다. </br>
/// - 기본 소리 출력 장치를 찾을 경우 `Ok(Some(...))`를 반환합니다. </br>
/// - 기본 소리 출력 장치가 없는 경우 `Ok(None)`를 반환합니다. </br>
/// - 기본 소리 출력 장치를 찾는 도중 오류가 발생한 경우 `Err`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Gets the user's default sound output device. </br>
/// - Returns `Ok(Some(...))`, if the default sound ouput device is found. </br>
/// - If there is no default sound output device, returns `Ok(None)` </br>
/// - Returns `Err` if an error occurs while finding the default sound output device. </br>
/// 
pub fn get_default_output_stream() -> AppResult<Option<(OutputStream, OutputStreamHandle)>> {
    match OutputStream::try_default() {
        Ok((stream, handle)) => {
            Ok(Some((stream, handle)))
        },
        Err(err) => match err {
            StreamError::NoDevice => {
                log::warn!("Default sound output device not found!");
                Ok(None)
            },
            _ => Err(game_err!(
                "Failed to find default sound output device",
                "The following error occurred while searching for a sound output device: {}", 
                err.to_string()
            ))
        }
    }
}

/// #### 한국어 </br>
/// 새로운 소리 제어자를 생성합니다. </br>
/// - 소리 제어자 생성에 성공할 경우 `Ok(Some(...))`을 반환합니다. </br>
/// - 기본 소리 출력 장치를 사용할 수 없는 경우 `Ok(None)`을 반환합니다. </br> 
/// - 소리 제어자 생성 중 오류가 발생한 경우 `Err`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a new sound controller. </br>
/// - If sound controller creation is successful, it returns `Ok(Some(...))`. </br>
/// - Returns `Ok(None)` if the default sound output device is not available. </br>
/// - If an error occurs while creating a sound controller, `Err` is returned. </br>
/// 
pub fn try_new_sink(stream_handle: &OutputStreamHandle) -> AppResult<Option<Sink>> {
    match Sink::try_new(&stream_handle) {
        Ok(sink) => Ok(Some(sink)), 
        Err(err) => match err {
            PlayError::NoDevice => {
                log::warn!("Default sound output device is not available!");
                Ok(None)
            }, 
            _ => Err(game_err!(
                "Failed to create sound controller", 
                "Sound controller creation failed for the following reasons: {}", 
                err.to_string()
            ))
        }
    }
}
