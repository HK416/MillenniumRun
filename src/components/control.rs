use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use winit::keyboard::KeyCode;


lazy_static! {
    static ref CODE_MAP: HashMap<Code, KeyCode> = HashMap::from_iter([
        (Code::KeyA, KeyCode::KeyA),
        (Code::KeyB, KeyCode::KeyB),
        (Code::KeyC, KeyCode::KeyC),
        (Code::KeyD, KeyCode::KeyD),
        (Code::KeyE, KeyCode::KeyE),
        (Code::KeyF, KeyCode::KeyF),
        (Code::KeyG, KeyCode::KeyG),
        (Code::KeyH, KeyCode::KeyH),
        (Code::KeyI, KeyCode::KeyI),
        (Code::KeyJ, KeyCode::KeyJ),
        (Code::KeyK, KeyCode::KeyK),
        (Code::KeyL, KeyCode::KeyL),
        (Code::KeyM, KeyCode::KeyM),
        (Code::KeyN, KeyCode::KeyN),
        (Code::KeyO, KeyCode::KeyO),
        (Code::KeyP, KeyCode::KeyP),
        (Code::KeyQ, KeyCode::KeyQ),
        (Code::KeyR, KeyCode::KeyR),
        (Code::KeyS, KeyCode::KeyS),
        (Code::KeyT, KeyCode::KeyT),
        (Code::KeyU, KeyCode::KeyU),
        (Code::KeyV, KeyCode::KeyV),
        (Code::KeyW, KeyCode::KeyW),
        (Code::KeyX, KeyCode::KeyX),
        (Code::KeyY, KeyCode::KeyY),
        (Code::KeyZ, KeyCode::KeyZ),
        (Code::ArrowDown, KeyCode::ArrowDown),
        (Code::ArrowLeft, KeyCode::ArrowLeft),
        (Code::ArrowRight, KeyCode::ArrowRight),
        (Code::ArrowUp, KeyCode::ArrowUp),
        (Code::Numpad0, KeyCode::Numpad0),
        (Code::Numpad1, KeyCode::Numpad1),
        (Code::Numpad2, KeyCode::Numpad2),
        (Code::Numpad3, KeyCode::Numpad3),
        (Code::Numpad4, KeyCode::Numpad4),
        (Code::Numpad5, KeyCode::Numpad5),
        (Code::Numpad6, KeyCode::Numpad6),
        (Code::Numpad7, KeyCode::Numpad7),
        (Code::Numpad8, KeyCode::Numpad8),
        (Code::Numpad9, KeyCode::Numpad9),
    ]);
}



/// #### 한국어 </br>
/// 사용자가 게임 조작에 할당할 수 있는 키보드 자판의 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// A list of keyboard keys that the user can assign to an game controls. </br>
/// 
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Code {
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
}

impl Code {
    #[inline]
    pub fn to_keycode(self) -> KeyCode {
        *CODE_MAP.get(&self).expect("Registered key code not found!")
    }
}



/// #### 한국어 </br>
/// 게임에서 사용하는 조작키를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the control keys used in the game. </br>
/// 
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Control {
    pub up: Code, 
    pub down: Code, 
    pub left: Code, 
    pub right: Code, 
}

impl Default for Control {
    #[inline]
    fn default() -> Self {
        Self { 
            up: Code::KeyW, 
            down: Code::KeyS, 
            left: Code::KeyA, 
            right: Code::KeyD, 
        }
    }
}

