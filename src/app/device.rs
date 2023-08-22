use std::time::Instant;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};
use std::collections::HashMap;

use lazy_static::lazy_static;
use crossbeam::queue::ArrayQueue;

use super::GameTimer;



/// #### 한국어
/// 키보드의 식별 코드 입니다. 
/// [`winit::event::VirtualKeyCode`]의 래퍼 타입입니다.
/// 
/// #### English (Translation)
/// This is the identification code of the keyboard.
/// This is a wrapper type for [`winit::event::VirtualKeyCode`].
/// 
pub type KeyCode = winit::event::VirtualKeyCode;

/// #### 한국어
/// 마우스 버튼의 식별 코드 입니다. 
/// [`winit::event::MouseButton`]의 래퍼 타입입니다.
/// 
/// #### English (Translation)
/// This is the identification code of the mouse button.
/// This is a wrapper type for [`winit::event::MouseButton`].
/// 
pub type MouseButton = winit::event::MouseButton;


pub const MAX_HISTORY_SIZE: usize = 50;


lazy_static! {
    static ref KEYBOARD_STATE: Arc<KeyboardStateInner> = Arc::new(KeyboardStateInner::new(MAX_HISTORY_SIZE));
}



/// #### 한국어
/// `KeyboardState`의 내부 데이터 입니다.
/// 어플리케이션 실행 이후 키보드의 상태를 저장합니다.
/// 
/// #### English (Translation)
/// This is the internal data of `KeyboardState`.
/// Stores the state of the keyboard after running the application.
/// 
#[derive(Debug)]
struct KeyboardStateInner {
    state: HashMap<KeyCode, (RwLock<Option<Instant>>, AtomicBool)>,
    history: ArrayQueue<(KeyCode, bool)>,
}

impl KeyboardStateInner {
    #[inline]
    pub fn new(history_capacity: usize) -> Self {
        Self {
            state: [
                (KeyCode::Key1, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key2, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key3, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key4, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key5, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key6, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key7, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key8, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key9, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Key0, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::A, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::B, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::C, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::D, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::E, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::G, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::H, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::I, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::J, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::K, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::L, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::M, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::N, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::O, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::P, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Q, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::R, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::S, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::T, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::U, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::V, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::W, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::X, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Y, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Z, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Escape, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F1, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F2, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F3, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F4, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F5, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F6, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F7, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F8, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F9, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F10, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F11, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F12, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F13, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F14, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F15, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F16, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F17, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F18, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F19, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F20, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F21, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F22, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F23, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::F24, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Snapshot, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Scroll, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Pause, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Insert, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Home, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Delete, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::End, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::PageDown, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::PageUp, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Left, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Up, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Right, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Down, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Back, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Return, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Space, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Compose, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Caret, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numlock, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad0, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad1, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad2, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad3, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad4, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad5, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad6, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad7, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad8, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Numpad9, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadAdd, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadDivide, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadDecimal, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadComma, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadEnter, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadEquals, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadMultiply, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NumpadSubtract, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::AbntC1, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::AbntC2, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Apostrophe, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Apps, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Asterisk, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::At, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Ax, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Backslash, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Calculator, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Capital, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Colon, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Comma, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Convert, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Equals, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Grave, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Kana, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Kanji, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::LAlt, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::LBracket, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::LControl, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::LShift, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::LWin, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Mail, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::MediaSelect, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::MediaStop, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Minus, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Mute, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::MyComputer, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NavigateForward, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NavigateBackward, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NextTrack, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::NoConvert, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::OEM102, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Period, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::PlayPause, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Plus, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Power, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::PrevTrack, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::RAlt, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::RBracket, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::RControl, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::RShift, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::RWin, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Semicolon, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Slash, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Sleep, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Stop, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Sysrq, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Tab, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Underline, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Unlabeled, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::VolumeDown, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::VolumeUp, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Wake, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebBack, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebFavorites, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebForward, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebHome, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebRefresh, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebSearch, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::WebStop, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Yen, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Copy, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Paste, (RwLock::new(None), AtomicBool::new(false))),
                (KeyCode::Cut, (RwLock::new(None), AtomicBool::new(false))),
            ].into_iter().collect(),
            history: ArrayQueue::new(history_capacity),
        }
    }

    /// #### 한국어
    /// 키보드의 키가 눌렸을때 호출되는 함수 입니다.
    /// 키보드 상태를 변경하고 기록합니다.
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is pressed.
    /// Change and record keyboard state.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 1. 내부 뮤텍스 잠금에 실패할 경우 프로그램 실행을 중단시킵니다.
    /// 자세한 내용은 [`std::sync::RwLock`]을 참고하세요.
    /// 2. 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// 1. Abort program execution if the internal mutex lock fails.
    /// See [`std::sync::RwLock`] for details.
    /// 2. Abort program execution if the `KeyCode` is invalid.
    /// 
    fn on_pressed(&self, timer: &GameTimer, keycode: KeyCode) {
        let (beg_time_point, state) = self.state.get(&keycode).expect("Invalid Keycode!");
        let mut beg_time_point = beg_time_point.write().expect("Failed to access keyboard time points.");
        if beg_time_point.is_none() {
            *beg_time_point = Some(timer.current_time_point());
            state.store(true, MemOrdering::Release);
            self.history.force_push((keycode, true));
        }
    }

    /// #### 한국어
    /// 키보드의 키가 떼어졌을 때 호출되는 함수 입니다.
    /// 키보드의 상태를 변경하고 기록합니다.
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is released.
    /// Change and record keyboard state.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 1. 내부 뮤텍스 잠금에 실패할 경우 프로그램 실행을 중단시킵니다.
    /// 자세한 내용은 [`std::sync::RwLock`]을 참고하세요.
    /// 2. 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// 1. Abort program execution if the internal mutex lock fails.
    /// See [`std::sync::RwLock`] for details.
    /// 2. Abort program execution if the `KeyCode` is invalid.
    /// 
    fn on_released(&self, keycode: KeyCode) {
        let (beg_time_point, state) = self.state.get(&keycode).expect("Invalid Keycode!");
        *beg_time_point.write().expect("Failed to access keyboard time points.") = None;
        state.store(false, MemOrdering::Release);
        self.history.force_push((keycode, false));
    }

    /// #### 한국어
    /// 주어진 `KeyCode`의 키가 눌렸을 경우 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns `true` if a key with the given `KeyCode` is pressed.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// Abort program execution if the `KeyCode` is invalid.
    /// 
    fn is_pressed(&self, keycode: KeyCode) -> bool {
        let (_, state) = self.state.get(&keycode).expect("Invalid Keycode!");
        state.load(MemOrdering::Acquire)
    }

    /// #### 한국어
    /// 주어진 `KeyCode`의 키가 눌렸을 경우 눌려진 시간을 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns the pressed time if a key with the given `KeyCode` is pressed.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// Abort program execution if the `KeyCode` is invalid.
    /// 
    fn pressed_duration_sec(&self, timer: &GameTimer, keycode: KeyCode) -> Option<f64> {
        let (beg_time_point, _) = self.state.get(&keycode).expect("Invalid Keycode!");
        match *beg_time_point.read().expect("Failed to access keyboard time points.") {
            Some(beg_time_point) => Some(
                timer.current_time_point()
                .saturating_duration_since(beg_time_point)
                .as_secs_f64()
            ),
            None => None,
        }
    }
}



#[derive(Debug, Clone)]
pub struct KeyboardState;

impl KeyboardState {
    /// #### 한국어
    /// 키보드의 키가 눌렸을때 호출되는 함수 입니다.
    /// 키보드 상태를 변경하고 기록합니다.
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is pressed.
    /// Change and record keyboard state.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 1. 내부 뮤텍스 잠금에 실패할 경우 프로그램 실행을 중단시킵니다.
    /// 자세한 내용은 [`std::sync::RwLock`]을 참고하세요.
    /// 2. 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// 1. Abort program execution if the internal mutex lock fails.
    /// See [`std::sync::RwLock`] for details.
    /// 2. Abort program execution if the `KeyCode` is invalid.
    /// 
    #[inline]
    pub fn on_pressed<K: Into<KeyCode>>(timer: &GameTimer, keycode: K) {
        KEYBOARD_STATE.on_pressed(timer, keycode.into())
    }

    /// #### 한국어
    /// 키보드의 키가 떼어졌을 때 호출되는 함수 입니다.
    /// 키보드의 상태를 변경하고 기록합니다.
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is released.
    /// Change and record keyboard state.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 1. 내부 뮤텍스 잠금에 실패할 경우 프로그램 실행을 중단시킵니다.
    /// 자세한 내용은 [`std::sync::RwLock`]을 참고하세요.
    /// 2. 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// 1. Abort program execution if the internal mutex lock fails.
    /// See [`std::sync::RwLock`] for details.
    /// 2. Abort program execution if the `KeyCode` is invalid.
    /// 
    #[inline]
    pub fn on_released<K: Into<KeyCode>>(keycode: K) {
        KEYBOARD_STATE.on_released(keycode.into())
    }

    /// #### 한국어
    /// 주어진 `KeyCode`의 키가 눌렸을 경우 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns `true` if a key with the given `KeyCode` is pressed.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// Abort program execution if the `KeyCode` is invalid.
    /// 
    #[inline]
    pub fn is_pressed(keycode: KeyCode) -> bool {
        KEYBOARD_STATE.is_pressed(keycode)
    }

    /// #### 한국어
    /// 주어진 `KeyCode`의 키가 눌렸을 경우 눌려진 시간을 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns the pressed time if a key with the given `KeyCode` is pressed.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 유효하지 않은 `KeyCode`일 경우 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// Abort program execution if the `KeyCode` is invalid.
    /// 
    #[inline]
    pub fn pressed_duration_sec(timer: &GameTimer, keycode: KeyCode) -> Option<f64> {
        KEYBOARD_STATE.pressed_duration_sec(timer, keycode)
    }
}
