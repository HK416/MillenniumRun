#[cfg(debug_assertions)]
mod dev;

mod entry;
mod enter_msgbox;
mod enter_stage;
mod enter_selected;
mod enter_setting;
mod exit_msgbox;
mod exit_stage;
mod exit_selected;
mod exit_setting;
mod menu;
mod msgbox;
mod stage;
mod selected;
mod setting;

use std::thread;

use winit::event::Event;
use rodio::OutputStreamHandle;

use crate::{
    assets::bundle::AssetBundle,
    components::{sound::SoundDecoder, user::Settings},
    nodes::{path, title::TitleScene},
    system::{error::AppResult, event::AppEvent, shared::Shared},
};



/// #### 한국어 </br>
/// `title` 게임 장면의 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// Status list of `title` game scene. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleState {
    #[cfg(debug_assertions)]
    Dev,

    #[default]
    Entry,
    Menu,
    EnterSetting,
    ExitSetting,
    Setting,
    EnterMsgBox,
    ExitMsgBox,
    MsgBox,
    EnterStage,
    ExitStage,
    Stage,
    EnterSelected,
    ExitSelected,
    Selected,
    NumStatus,
}



type HandleEventsFn = dyn Fn(&mut TitleScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut TitleScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&TitleScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; TitleState::NumStatus as usize] = [
    #[cfg(debug_assertions)]
    &dev::handle_events,
    
    &entry::handle_events,
    &menu::handle_events,
    &enter_setting::handle_events,
    &exit_setting::handle_events,
    &setting::handle_events,
    &enter_msgbox::handle_events,
    &exit_msgbox::handle_events,
    &msgbox::handle_events,
    &enter_stage::handle_events,
    &exit_stage::handle_events,
    &stage::handle_events,
    &enter_selected::handle_events,
    &exit_selected::handle_events,
    &selected::handle_events,
];

pub const UPDATES: [&'static UpdateFn; TitleState::NumStatus as usize] = [
    #[cfg(debug_assertions)]
    &dev::update,

    &entry::update,
    &menu::update,
    &enter_setting::update,
    &exit_setting::update,
    &setting::update,
    &enter_msgbox::update,
    &exit_msgbox::update,
    &msgbox::update,
    &enter_stage::update,
    &exit_stage::update,
    &stage::update,
    &enter_selected::update,
    &exit_selected::update,
    &selected::update,
];

pub const DRAWS: [&'static DrawFn; TitleState::NumStatus as usize] = [
    #[cfg(debug_assertions)]
    &dev::draw,

    &entry::draw,
    &menu::draw,
    &enter_setting::draw,
    &exit_setting::draw,
    &setting::draw,
    &enter_msgbox::draw,
    &exit_msgbox::draw,
    &msgbox::draw,
    &enter_stage::draw,
    &exit_stage::draw,
    &stage::draw,
    &enter_selected::draw,
    &exit_selected::draw,
    &selected::draw,
];


/// #### 한국어 </br>
/// 클릭음을 재생합니다. </br>
/// 
/// #### English (Translation) </br>
/// Play a click sound. </br>
/// 
fn play_click_sound(_this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    use crate::components::sound::create_sink;

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let stream_handle = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 클릭 소리를 재생합니다.
    // (English Translation) Play a click sound.
    let source = asset_bundle.get(path::sys::CLICK_SOUND_PATH)?
        .read(&SoundDecoder)?;
    let sink = create_sink(stream_handle)?;
    sink.set_volume(settings.effect_volume.get_norm());
    thread::spawn(move || {
        sink.append(source);
        sink.sleep_until_end();
        sink.detach();
    });

    Ok(())
}

/// #### 한국어 </br>
/// 취소음을 재생합니다. </br>
/// 
/// #### English (Translation) </br>
/// Play a cancel sound. </br>
/// 
fn play_cancel_sound(_this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    use crate::components::sound::create_sink;

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let stream_handle = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 클릭 소리를 재생합니다.
    // (English Translation) Play a click sound.
    let source = asset_bundle.get(path::sys::CANCEL_SOUND_PATH)?
        .read(&SoundDecoder)?;
    let sink = create_sink(stream_handle)?;
    sink.set_volume(settings.effect_volume.get_norm());
    thread::spawn(move || {
        sink.append(source);
        sink.sleep_until_end();
        sink.detach()
    });

    Ok(())
}
