mod run;

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
    Run,
}

type HandleEventsFn = dyn Fn(&mut InGameScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut InGameScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&InGameScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; 1] = [
    &run::handle_events,
];

pub const UPDATES: [&'static UpdateFn; 1] = [
    &run::update,
];

pub const DRAWS: [&'static DrawFn; 1] = [
    &run::draw,
];
