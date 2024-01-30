mod enter;
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
mod return_stage;

use winit::event::Event;

use crate::{
    nodes::title::TitleScene,
    system::{
        error::AppResult, 
        event::AppEvent, 
        shared::Shared
    },
};



/// #### 한국어 </br>
/// `title` 게임 장면의 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// Status list of `title` game scene. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleState {
    #[default]
    Enter,
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
    ReturnStage, 
}



type HandleEventsFn = dyn Fn(&mut TitleScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut TitleScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&TitleScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; 15] = [
    &enter::handle_events,
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
    &return_stage::handle_events, 
];

pub const UPDATES: [&'static UpdateFn; 15] = [
    &enter::update,
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
    &return_stage::update, 
];

pub const DRAWS: [&'static DrawFn; 15] = [
    &enter::draw,
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
    &return_stage::draw, 
];
