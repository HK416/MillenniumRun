mod enter; 
mod spawn; 
mod ready;
mod run; 
mod enter_pause;
mod pause;
mod exit_pause;
mod enter_msgbox;
mod msgbox;
mod exit_msgbox;
mod enter_setting;
mod setting;
mod exit_setting;
mod wait_for_finish;
mod disappear_run;
mod appear_result;
mod result;

use winit::event::Event;

use crate::{
    nodes::in_game::InGameScene,
    system::{
        error::AppResult,
        event::AppEvent,
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// `InGame` 게임 장면의 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// State list of `InGame` game scene. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InGameState {
    #[default]
    Enter, 
    Spawn, 
    Ready, 
    Run,
    EnterPause, 
    Pause, 
    ExitPause, 
    EnterMsgBox,
    MsgBox,
    ExitMsgBox,
    EnterSetting,
    Setting,
    ExitSetting,
    WaitForFinish, 
    DisappearRun, 
    AppearResult, 
    Result, 
}

type HandleEventsFn = dyn Fn(&mut InGameScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut InGameScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&InGameScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; 17] = [
    &enter::handle_events, 
    &spawn::handle_events, 
    &ready::handle_events, 
    &run::handle_events,
    &enter_pause::handle_events, 
    &pause::handle_events, 
    &exit_pause::handle_events, 
    &enter_msgbox::handle_events, 
    &msgbox::handle_events, 
    &exit_msgbox::handle_events, 
    &enter_setting::handle_events,
    &setting::handle_events,
    &exit_setting::handle_events,
    &wait_for_finish::handle_events, 
    &disappear_run::handle_events, 
    &appear_result::handle_events, 
    &result::handle_events, 
];

pub const UPDATES: [&'static UpdateFn; 17] = [
    &enter::update, 
    &spawn::update, 
    &ready::update, 
    &run::update,
    &enter_pause::update, 
    &pause::update, 
    &exit_pause::update, 
    &enter_msgbox::update, 
    &msgbox::update, 
    &exit_msgbox::update, 
    &enter_setting::update,
    &setting::update,
    &exit_setting::update,
    &wait_for_finish::update, 
    &disappear_run::update, 
    &appear_result::update, 
    &result::update, 
];

pub const DRAWS: [&'static DrawFn; 17] = [
    &enter::draw, 
    &spawn::draw, 
    &ready::draw, 
    &run::draw,
    &enter_pause::draw, 
    &pause::draw, 
    &exit_pause::draw, 
    &enter_msgbox::draw, 
    &msgbox::draw, 
    &exit_msgbox::draw, 
    &enter_setting::draw,
    &setting::draw,
    &exit_setting::draw,
    &wait_for_finish::draw, 
    &disappear_run::draw, 
    &appear_result::draw, 
    &result::draw, 
];
