use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;

use lazy_static::lazy_static;
use winit::event::{MouseButton, VirtualKeyCode};



lazy_static! {
    static ref MOUSE_BTN_STATE: MouseButtonStateInner = MouseButtonStateInner::new();
    static ref KEYBOARD_STATE: KeyboardStateInner = KeyboardStateInner::new();
}



/// #### 한국어
/// `MouseButtonState`의 내부 데이터 입니다.
/// 어플리케이션 실행 이후 마우스 버튼의 상태를 저장합니다.
/// 
/// #### English (Translation)
/// This is the internal data of `MouseButtonState`.
/// Stores the state of the mouse button after running the application.
/// 
#[derive(Debug)]
struct MouseButtonStateInner(HashMap<MouseButton, AtomicBool>);

impl MouseButtonStateInner {
    #[inline]
    fn new() -> Self {
        Self(HashMap::from_iter([
            (MouseButton::Left, false.into()),
            (MouseButton::Right, false.into()),
            (MouseButton::Middle, false.into()),
        ]))
    }

    /// #### 한국어
    /// 마우스 버튼이 눌렸을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when the mouse button is pressed. </br>
    /// 
    fn on_pressed(&self, button: &MouseButton) {
        match button {
            MouseButton::Left | MouseButton::Right | MouseButton::Middle => {
                let mut state = self.0.get(button)
                    .expect("Invalid mouse button!")
                    .store(true, Ordering::Release);
            },
            _ => { /* empty */ }
        };
    }

    /// #### 한국어
    /// 마우스 버튼이 떼어졌을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when the mouse button is released. </br>
    /// 
    fn on_released(&self, button: &MouseButton) {
        match button {
            MouseButton::Left | MouseButton::Right | MouseButton::Middle => {
                let mut state = self.0.get(button)
                    .expect("Invalid mouse button!")
                    .store(false, Ordering::Release);
            },
            _ => { /* empty */}
        }
    }


    /// #### 한국어
    /// 주어진 마우스 버튼이 눌렸는지 확인하는 함수입니다. </br>
    /// [`Left`](winit::event::MouseButton), [`Right`](winit::event::MouseButton), [`Middle`](winit::event::MouseButton)만 확인이 가능하며
    /// 버튼이 눌렸을 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// This function checks whether the given mouse button has been pressed. </br>
    /// Only [`Left`](winit::event::MouseButton), [`Right`](winit::event::MouseButton), and [`Middle`](winit::event::MouseButton) can be checked,
    /// and if the button is pressed, `true` is returned. </br>
    /// 
    fn is_pressed(&self, button: &MouseButton) -> bool {
        match button {
            MouseButton::Left | MouseButton::Right | MouseButton::Middle => {
                self.0.get(button)
                    .expect("Invalid mouse button!")
                    .load(Ordering::Acquire)
            },
            _ => false,
        }
    }
}



#[derive(Debug)]
pub struct MouseButtonState;

impl MouseButtonState {
    /// #### 한국어
    /// 마우스 버튼이 눌렸을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when the mouse button is pressed. </br>
    /// 
    #[inline]
    pub fn on_pressed(button: &MouseButton) {
        MOUSE_BTN_STATE.on_pressed(button)
    }

    /// #### 한국어
    /// 마우스 버튼이 떼어졌을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when the mouse button is released. </br>
    /// 
    #[inline]
    pub fn on_released(button: &MouseButton) {
        MOUSE_BTN_STATE.on_released(button)
    }

    /// #### 한국어 </br>
    /// 주어진 마우스 버튼이 눌렸는지 확인하는 함수입니다. </br>
    /// [`Left`](winit::event::MouseButton), [`Right`](winit::event::MouseButton), [`Middle`](winit::event::MouseButton)만 확인이 가능하며
    /// 버튼이 눌렸을 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This function checks whether the given mouse button has been pressed. </br>
    /// Only [`Left`](winit::event::MouseButton), [`Right`](winit::event::MouseButton), and [`Middle`](winit::event::MouseButton) can be checked,
    /// and if the button is pressed, `true` is returned. </br>
    /// 
    #[inline]
    pub fn is_pressed(button: &MouseButton) -> bool {
        MOUSE_BTN_STATE.is_pressed(button)
    }
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
struct KeyboardStateInner(HashMap<VirtualKeyCode, AtomicBool>);

impl KeyboardStateInner {
    #[inline]
    fn new() -> Self {
        Self(HashMap::from_iter([
            (VirtualKeyCode::Key1, false.into()),
            (VirtualKeyCode::Key2, false.into()),
            (VirtualKeyCode::Key3, false.into()),
            (VirtualKeyCode::Key4, false.into()),
            (VirtualKeyCode::Key5, false.into()),
            (VirtualKeyCode::Key6, false.into()),
            (VirtualKeyCode::Key7, false.into()),
            (VirtualKeyCode::Key8, false.into()),
            (VirtualKeyCode::Key9, false.into()),
            (VirtualKeyCode::Key0, false.into()),
            (VirtualKeyCode::A, false.into()),
            (VirtualKeyCode::B, false.into()),
            (VirtualKeyCode::C, false.into()),
            (VirtualKeyCode::D, false.into()),
            (VirtualKeyCode::E, false.into()),
            (VirtualKeyCode::F, false.into()),
            (VirtualKeyCode::G, false.into()),
            (VirtualKeyCode::H, false.into()),
            (VirtualKeyCode::I, false.into()),
            (VirtualKeyCode::J, false.into()),
            (VirtualKeyCode::K, false.into()),
            (VirtualKeyCode::L, false.into()),
            (VirtualKeyCode::M, false.into()),
            (VirtualKeyCode::N, false.into()),
            (VirtualKeyCode::O, false.into()),
            (VirtualKeyCode::P, false.into()),
            (VirtualKeyCode::Q, false.into()),
            (VirtualKeyCode::R, false.into()),
            (VirtualKeyCode::S, false.into()),
            (VirtualKeyCode::T, false.into()),
            (VirtualKeyCode::U, false.into()),
            (VirtualKeyCode::V, false.into()),
            (VirtualKeyCode::W, false.into()),
            (VirtualKeyCode::X, false.into()),
            (VirtualKeyCode::Y, false.into()),
            (VirtualKeyCode::Z, false.into()),
            (VirtualKeyCode::Escape, false.into()),
            (VirtualKeyCode::F1, false.into()),
            (VirtualKeyCode::F2, false.into()),
            (VirtualKeyCode::F3, false.into()),
            (VirtualKeyCode::F4, false.into()),
            (VirtualKeyCode::F5, false.into()),
            (VirtualKeyCode::F6, false.into()),
            (VirtualKeyCode::F7, false.into()),
            (VirtualKeyCode::F8, false.into()),
            (VirtualKeyCode::F9, false.into()),
            (VirtualKeyCode::F10, false.into()),
            (VirtualKeyCode::F11, false.into()),
            (VirtualKeyCode::F12, false.into()),
            (VirtualKeyCode::F13, false.into()),
            (VirtualKeyCode::F14, false.into()),
            (VirtualKeyCode::F15, false.into()),
            (VirtualKeyCode::F16, false.into()),
            (VirtualKeyCode::F17, false.into()),
            (VirtualKeyCode::F18, false.into()),
            (VirtualKeyCode::F19, false.into()),
            (VirtualKeyCode::F20, false.into()),
            (VirtualKeyCode::F21, false.into()),
            (VirtualKeyCode::F22, false.into()),
            (VirtualKeyCode::F23, false.into()),
            (VirtualKeyCode::F24, false.into()),
            (VirtualKeyCode::Snapshot, false.into()),
            (VirtualKeyCode::Scroll, false.into()),
            (VirtualKeyCode::Pause, false.into()),
            (VirtualKeyCode::Insert, false.into()),
            (VirtualKeyCode::Home, false.into()),
            (VirtualKeyCode::Delete, false.into()),
            (VirtualKeyCode::End, false.into()),
            (VirtualKeyCode::PageDown, false.into()),
            (VirtualKeyCode::PageUp, false.into()),
            (VirtualKeyCode::Left, false.into()),
            (VirtualKeyCode::Up, false.into()),
            (VirtualKeyCode::Right, false.into()),
            (VirtualKeyCode::Down, false.into()),
            (VirtualKeyCode::Back, false.into()),
            (VirtualKeyCode::Return, false.into()),
            (VirtualKeyCode::Space, false.into()),
            (VirtualKeyCode::Compose, false.into()),
            (VirtualKeyCode::Caret, false.into()),
            (VirtualKeyCode::Numlock, false.into()),
            (VirtualKeyCode::Numpad0, false.into()),
            (VirtualKeyCode::Numpad1, false.into()),
            (VirtualKeyCode::Numpad2, false.into()),
            (VirtualKeyCode::Numpad3, false.into()),
            (VirtualKeyCode::Numpad4, false.into()),
            (VirtualKeyCode::Numpad5, false.into()),
            (VirtualKeyCode::Numpad6, false.into()),
            (VirtualKeyCode::Numpad7, false.into()),
            (VirtualKeyCode::Numpad8, false.into()),
            (VirtualKeyCode::Numpad9, false.into()),
            (VirtualKeyCode::NumpadAdd, false.into()),
            (VirtualKeyCode::NumpadDivide, false.into()),
            (VirtualKeyCode::NumpadDecimal, false.into()),
            (VirtualKeyCode::NumpadComma, false.into()),
            (VirtualKeyCode::NumpadEnter, false.into()),
            (VirtualKeyCode::NumpadEquals, false.into()),
            (VirtualKeyCode::NumpadMultiply, false.into()),
            (VirtualKeyCode::NumpadSubtract, false.into()),
            (VirtualKeyCode::AbntC1, false.into()),
            (VirtualKeyCode::AbntC2, false.into()),
            (VirtualKeyCode::Apostrophe, false.into()),
            (VirtualKeyCode::Apps, false.into()),
            (VirtualKeyCode::Asterisk, false.into()),
            (VirtualKeyCode::At, false.into()),
            (VirtualKeyCode::Ax, false.into()),
            (VirtualKeyCode::Backslash, false.into()),
            (VirtualKeyCode::Calculator, false.into()),
            (VirtualKeyCode::Capital, false.into()),
            (VirtualKeyCode::Colon, false.into()),
            (VirtualKeyCode::Comma, false.into()),
            (VirtualKeyCode::Convert, false.into()),
            (VirtualKeyCode::Equals, false.into()),
            (VirtualKeyCode::Grave, false.into()),
            (VirtualKeyCode::Kana, false.into()),
            (VirtualKeyCode::Kanji, false.into()),
            (VirtualKeyCode::LAlt, false.into()),
            (VirtualKeyCode::LBracket, false.into()),
            (VirtualKeyCode::LControl, false.into()),
            (VirtualKeyCode::LShift, false.into()),
            (VirtualKeyCode::LWin, false.into()),
            (VirtualKeyCode::Mail, false.into()),
            (VirtualKeyCode::MediaSelect, false.into()),
            (VirtualKeyCode::MediaStop, false.into()),
            (VirtualKeyCode::Minus, false.into()),
            (VirtualKeyCode::Mute, false.into()),
            (VirtualKeyCode::MyComputer, false.into()),
            (VirtualKeyCode::NavigateForward, false.into()),
            (VirtualKeyCode::NavigateBackward, false.into()),
            (VirtualKeyCode::NextTrack, false.into()),
            (VirtualKeyCode::NoConvert, false.into()),
            (VirtualKeyCode::OEM102, false.into()),
            (VirtualKeyCode::Period, false.into()),
            (VirtualKeyCode::PlayPause, false.into()),
            (VirtualKeyCode::Plus, false.into()),
            (VirtualKeyCode::Power, false.into()),
            (VirtualKeyCode::PrevTrack, false.into()),
            (VirtualKeyCode::RAlt, false.into()),
            (VirtualKeyCode::RBracket, false.into()),
            (VirtualKeyCode::RControl, false.into()),
            (VirtualKeyCode::RShift, false.into()),
            (VirtualKeyCode::RWin, false.into()),
            (VirtualKeyCode::Semicolon, false.into()),
            (VirtualKeyCode::Slash, false.into()),
            (VirtualKeyCode::Sleep, false.into()),
            (VirtualKeyCode::Stop, false.into()),
            (VirtualKeyCode::Sysrq, false.into()),
            (VirtualKeyCode::Tab, false.into()),
            (VirtualKeyCode::Underline, false.into()),
            (VirtualKeyCode::Unlabeled, false.into()),
            (VirtualKeyCode::VolumeDown, false.into()),
            (VirtualKeyCode::VolumeUp, false.into()),
            (VirtualKeyCode::Wake, false.into()),
            (VirtualKeyCode::WebBack, false.into()),
            (VirtualKeyCode::WebFavorites, false.into()),
            (VirtualKeyCode::WebForward, false.into()),
            (VirtualKeyCode::WebHome, false.into()),
            (VirtualKeyCode::WebRefresh, false.into()),
            (VirtualKeyCode::WebSearch, false.into()),
            (VirtualKeyCode::WebStop, false.into()),
            (VirtualKeyCode::Yen, false.into()),
            (VirtualKeyCode::Copy, false.into()),
            (VirtualKeyCode::Paste, false.into()),
            (VirtualKeyCode::Cut, false.into()),
        ]))
    }

    /// #### 한국어
    /// 키보드의 키가 눌렸을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is pressed. </br>
    /// 
    fn on_pressed(&self, keycode: &VirtualKeyCode) {
        let mut state = self.0
            .get(keycode)
            .expect("Invalid key code!")
            .store(true, Ordering::Release);
    }

    /// #### 한국어
    /// 키보드의 키가 떼어졌을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is released. </br>
    /// 
    fn on_released(&self, keycode: &VirtualKeyCode) {
        let mut state = self.0.get(keycode)
            .expect("Invalid key code!")
            .store(false, Ordering::Release);
    }

    /// #### 한국어
    /// 주어진 가상 키 코드가 눌렸는지 확인하는 함수입니다. </br>
    /// 버튼이 눌렸을 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// This function checks whether the given virtual key code has been pressed. </br>
    /// if the button is pressed, `true` is returned. </br>
    /// 
    fn is_pressed(&self, keycode: &VirtualKeyCode) -> bool {
        self.0.get(keycode)
            .expect("Invalid key code!")
            .load(Ordering::Acquire)
    }
}



#[derive(Debug)]
pub struct KeyboardState;

impl KeyboardState {
    /// #### 한국어
    /// 키보드의 키가 눌렸을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is pressed. </br>
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 다음과 같은 경우 현재 스레드의 프로그램 실행을 중단합니다. </br>
    /// - 올바르지 않은 가상 키 코드가 주어졌을 경우. </br>
    /// - 내부 뮤텍스 잠금에 실패한 경우. </br>
    /// 
    /// #### English (Translation)
    /// Abort program execution in the current thread in the following cases: </br>
    /// - If an incorrect virtual key code is given. </br>
    /// - If locking an internal mutex fails. </br>
    /// 
    #[inline]
    pub fn on_pressed(keycode: &VirtualKeyCode) {
        KEYBOARD_STATE.on_pressed(keycode)
    }

    /// #### 한국어
    /// 키보드의 키가 떼어졌을 때 호출되는 함수입니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when a key on the keyboard is released. </br>
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 다음과 같은 경우 현재 스레드의 프로그램 실행을 중단합니다. </br>
    /// - 올바르지 않은 가상 키 코드가 주어졌을 경우. </br>
    /// - 내부 뮤텍스 잠금에 실패한 경우. </br>
    /// 
    /// #### English (Translation)
    /// Abort program execution in the current thread in the following cases: </br>
    /// - If an incorrect virtual key code is given. </br>
    /// - If locking an internal mutex fails. </br>
    /// 
    #[inline]
    pub fn on_released(keycode: &VirtualKeyCode) {
        KEYBOARD_STATE.on_released(keycode)
    }

    /// #### 한국어
    /// 주어진 가상 키 코드가 눌렸는지 확인하는 함수입니다. </br>
    /// 버튼이 눌렸을 경우 버튼이 눌린 시점을 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// This function checks whether the given virtual key code has been pressed. </br>
    /// if the button is pressed, the time when the button was pressed is returned. </br>
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 다음과 같은 경우 현재 스레드의 프로그램 실행을 중단합니다. </br>
    /// - 올바르지 않은 가상 키 코드가 주어졌을 경우. </br>
    /// - 내부 뮤텍스 잠금에 실패한 경우. </br>
    /// 
    /// #### English (Translation)
    /// Abort program execution in the current thread in the following cases: </br>
    /// - If an incorrect virtual key code is given. </br>
    /// - If locking an internal mutex fails. </br>
    /// 
    #[inline]
    pub fn is_pressed(keycode: &VirtualKeyCode) -> bool {
        KEYBOARD_STATE.is_pressed(keycode)
    }
}
