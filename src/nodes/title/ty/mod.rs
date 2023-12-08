mod background;
mod sprite;
mod system;
mod menu;
mod exit;
mod stage;
mod setting;

pub use self::{
    background::{Backgrounds, BackgroundTags, BackgroundDesc},
    sprite::{SpriteButtons, SpriteButtonTags, SpriteButtonDesc},
    system::{SystemButtons, SystemButtonTags, SystemButtonDesc},
    menu::{MenuButtons, MenuButtonTags, MenuButtonDesc},
    exit::{ExitWindow, ExitWindowTags, ExitWindowDesc},
    stage::{StageWindow, StageWindowTags, StageWindowDesc},
    setting::{SettingWindow, SettingWindowTags, SettingWindowDesc},
};
