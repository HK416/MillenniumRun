//! #### 한국어 </br>
//! `Intro` 게임 장면의 상태의 순서는 다음과 같습니다: </br>
//! 1. FadeIn </br>
//! 2. DisplayNotify </br>
//! 3. DisapperNotify </br>
//! 4. PlaytTitleVoice </br>
//! 5. AppearLogo </br>
//! 6. DisplayLogo </br>
//! 7. WaitLoading </br>
//! 8. FadeOut </br>
//! 
//! #### English (Translation) </br> 
//! The order of states in the `Intro` game scene is as follows: </br>
//! 1. FadeIn </br>
//! 2. DisplayNotify </br>
//! 3. DisapperNotify </br>
//! 4. PlaytTitleVoice </br>
//! 5. AppearLogo </br>
//! 6. DisplayLogo </br>
//! 7. WaitLoading </br>
//! 8. FadeOut </br>
//! 
mod fade_in;
mod display_notify;
mod disappear_notify;
mod play_title_voice;
mod appear_logo;
mod display_logo;
mod wait_loading;
mod fade_out;

use crate::{
    nodes::intro::IntroScene,
    system::{
        error::AppResult,
        shared::Shared,
    },
};

type UpdateFn = dyn Fn(&mut super::IntroScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&IntroScene, &mut Shared) -> AppResult<()>;

/// #### (한국어) </br>
/// `intro` 게임 장면의 상태별 갱신 함수입니다. </br>
///  
/// #### English (Translation) </br>
/// This is a updating function for each state of the `intro` game scene.
/// 
pub const UPDATE: [&'static UpdateFn; 8] = [
    &fade_in::update,
    &display_notify::update,
    &disappear_notify::update,
    &play_title_voice::update,
    &appear_logo::update,
    &display_logo::update,
    &wait_loading::update,
    &fade_out::update,
];

/// #### (한국어) </br>
/// `intro` 게임 장면의 상태별 그리기 함수입니다. </br>
///  
/// #### English (Translation) </br>
/// This is a drawing function for each state of the `intro` game scene.
/// 
pub const DRAW: [&'static DrawFn; 8] = [
    &fade_in::draw,
    &display_notify::draw,
    &disappear_notify::draw,
    &play_title_voice::draw,
    &appear_logo::draw,
    &display_logo::draw,
    &wait_loading::draw,
    &fade_out::draw,
];



/// #### 한국어 </br>
/// 인트로 게임 장면의 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// Status list of `intro` game scene. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntroState {
    #[default]
    FadeIn,
    DisplayNotify,
    DisappearNotify,
    PlayTitleVoice,
    AppearLogo,
    DisplayLogo,
    WaitLoading,
    FadeOut,
}
