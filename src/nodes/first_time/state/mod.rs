mod wait;
mod exit;

use winit::event::Event;

use crate::{
    nodes::first_time::FirstTimeSetupScene,
    system::{error::AppResult, event::AppEvent, shared::Shared},
};


/// #### 한국어 </br>
/// `FirstTimeSetup` 게임 장면의 상태 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of status in the `FirstTimeSetup` game scene. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FirstTimeSetupSceneState {
    #[default]
    Wait,
    Exit,
    NumStatus
}

type HandleEventsFn = dyn Fn(&mut FirstTimeSetupScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut FirstTimeSetupScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&FirstTimeSetupScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; FirstTimeSetupSceneState::NumStatus as usize] = [
    &wait::handle_events,
    &exit::handle_events,
];

pub const UPDATES: [&'static UpdateFn; FirstTimeSetupSceneState::NumStatus as usize] = [
    &wait::update,
    &exit::update,
];

pub const DRAWS: [&'static DrawFn; FirstTimeSetupSceneState::NumStatus as usize] = [
    &wait::draw,
    &exit::draw,
];
