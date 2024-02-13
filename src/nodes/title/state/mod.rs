mod enter;
mod enter_msgbox;
mod enter_stage;
mod enter_selected;
mod enter_setting;
mod enter_viewer;
mod exit_msgbox;
mod exit_stage;
mod exit_selected;
mod exit_setting;
mod exit_viewer;
mod menu;
mod msgbox;
mod stage;
mod selected;
mod setting;
mod viewer;
mod return_stage;
mod tutorial0;
mod tutorial1;
mod tutorial2;
mod tutorial3;

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
    EnterViewer, 
    ExitViewer, 
    Viewer,
    ReturnStage, 
    Tutorial0, 
    Tutorial1, 
    Tutorial2, 
    Tutorial3, 
}



type HandleEventsFn = dyn Fn(&mut TitleScene, &mut Shared, Event<AppEvent>) -> AppResult<()>;
type UpdateFn = dyn Fn(&mut TitleScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFn = dyn Fn(&TitleScene, &mut Shared) -> AppResult<()>;

pub const HANDLE_EVENTS: [&'static HandleEventsFn; 22] = [
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
    &enter_viewer::handle_events, 
    &exit_viewer::handle_events, 
    &viewer::handle_events, 
    &return_stage::handle_events, 
    &tutorial0::handle_events, 
    &tutorial1::handle_events, 
    &tutorial2::handle_events, 
    &tutorial3::handle_events, 
];

pub const UPDATES: [&'static UpdateFn; 22] = [
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
    &enter_viewer::update, 
    &exit_viewer::update, 
    &viewer::update, 
    &return_stage::update, 
    &tutorial0::update, 
    &tutorial1::update, 
    &tutorial2::update, 
    &tutorial3::update, 
];

pub const DRAWS: [&'static DrawFn; 22] = [
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
    &enter_viewer::draw, 
    &exit_viewer::draw, 
    &viewer::draw, 
    &return_stage::draw, 
    &tutorial0::draw, 
    &tutorial1::draw, 
    &tutorial2::draw, 
    &tutorial3::draw, 
];
