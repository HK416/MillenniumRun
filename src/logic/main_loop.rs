use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};

use winit::window::Window;

use crate::{
    assets::AssetBundle,
    app::{GameTimer, GameLogicEvent, AppCmd, KeyboardState}, 
};


const UPDATE_FRAME_RATE: f64 = 1_000.0 / 60.0;



/// #### 한국어
/// 게임 로직 루프 함수입니다.
/// 
/// #### English (Translation)
/// This is the game logic loop function.
/// 
pub fn game_logic_loop(
    window: Arc<Window>,
    event_receiver: Receiver<GameLogicEvent>,
    message_sender: Sender<AppCmd>,
    asset_bundle: AssetBundle,
    running_flag: Arc<AtomicBool>,
) {
    log::info!("Run :: Game logic loop.");
    let mut elapsed_time_sec = 0.0;
    let mut timer = GameTimer::new();
    'logic_loop: while let Ok(event) = event_receiver.recv() {
        match event {
            GameLogicEvent::NextMainEvents => {
                timer.tick(None);
            },
            GameLogicEvent::MainEventsCleared => {
                elapsed_time_sec += timer.elapsed_time_sec_f64();
                while elapsed_time_sec > UPDATE_FRAME_RATE {
                    elapsed_time_sec -= UPDATE_FRAME_RATE;
                    log::debug!("update game logic!");
                }
            }
            GameLogicEvent::ApplicationTerminate => {
                break 'logic_loop;
            },
            GameLogicEvent::KeyPressed(keycode) => {
                KeyboardState::on_pressed(&timer, keycode);
            },
            GameLogicEvent::KeyReleased(keycode) => {
                KeyboardState::on_released(keycode)
            }
            _ => { }
        }
    }
    log::info!("End :: Game logic loop.");
}
