mod enter; 
mod spawn; 
mod ready;
mod run; 
mod enter_pause;
mod pause;
mod exit_pause;

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
}

type HandleEventsFn = dyn Fn(&mut InGameScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut InGameScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&InGameScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; 7] = [
    &enter::handle_events, 
    &spawn::handle_events, 
    &ready::handle_events, 
    &run::handle_events,
    &enter_pause::handle_events, 
    &pause::handle_events, 
    &exit_pause::handle_events, 
];

pub const UPDATES: [&'static UpdateFn; 7] = [
    &enter::update, 
    &spawn::update, 
    &ready::update, 
    &run::update,
    &enter_pause::update, 
    &pause::update, 
    &exit_pause::update, 
];

pub const DRAWS: [&'static DrawFn; 7] = [
    &enter::draw, 
    &spawn::draw, 
    &ready::draw, 
    &run::draw,
    &enter_pause::draw, 
    &pause::draw, 
    &exit_pause::draw, 
];
