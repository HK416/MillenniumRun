use std::io::Cursor;

use serde::{Serialize, Deserialize};
use rodio::{
    Sink, 
    Sample,
    Source,
    Decoder, 
    OutputStreamHandle, 
    cpal::FromSample,
};

use crate::{
    game_err,
    assets::{
        bundle::AssetBundle, 
        interface::AssetDecoder, 
    },
    components::user::Settings,
    system::{
        error::{AppResult, GameError}, 
        shared::Shared
    }, 
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

#[allow(dead_code)]
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
/// 주어진 소리를 재생합니다. </br>
/// 
/// #### English (Translation) </br>
/// Play the given sound. </br>
/// 
#[inline]
pub fn play_sound<S>(
    volume: Volume,
    source: S,
    stream: &OutputStreamHandle
) -> AppResult<Sink> 
where 
    S: Source + Send + 'static,
    f32: FromSample<S::Item>,
    S::Item: Sample + Send,
{
    let sink = Sink::try_new(stream)
        .map_err(|err| game_err!(
            "Sound player creation failed",
            "Sound player creation failed for following reasons: {}",
            err.to_string()
        ))?;
    sink.set_volume(volume.norm());
    sink.append(source);
    return Ok(sink);
}



/// #### 한국어 </br>
/// 클릭음을 재생하는 유틸리티 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a utility function that plays a click sound. </br>
/// 
#[inline]
pub fn play_click_sound(shared: &Shared) -> AppResult<()> {
    use std::thread;
    use crate::nodes::path;

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use. 
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 클릭 소리를 로드하고, 재생합니다.
    // (English Translation) Load and play the click sound. 
    let source = asset_bundle.get(path::CLICK_SOUND_PATH)?
        .read(&SoundDecoder)?;
    let sink = play_sound(
        settings.effect_volume, 
        source, 
        stream
    )?;

    // (한국어) 새로운 스레드에서 재생이 끝날 때까지 기다립니다.
    // (English Translation) Wait for playback to finish in a new thread.
    thread::spawn(move || {
        sink.sleep_until_end();
        sink.detach();
    });

    Ok(())
}



/// #### 한국어 </br>
/// 취소음을 재생하는 유틸리티 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a utility function that plays a cancel sound. </br>
/// 
#[inline]
pub fn play_cancel_sound(shared: &Shared) -> AppResult<()> {
    use std::thread;
    use crate::nodes::path;

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use. 
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 클릭 소리를 로드하고, 재생합니다.
    // (English Translation) Load and play the click sound. 
    let source = asset_bundle.get(path::CANCEL_SOUND_PATH)?
        .read(&SoundDecoder)?;
    let sink = play_sound(
        settings.effect_volume, 
        source, 
        stream
    )?;

    // (한국어) 새로운 스레드에서 재생이 끝날 때까지 기다립니다.
    // (English Translation) Wait for playback to finish in a new thread.
    thread::spawn(move || {
        sink.sleep_until_end();
        sink.detach();
    });

    Ok(())
}
