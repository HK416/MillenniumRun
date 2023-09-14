use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::VecDeque;

use winit::window::Window;

use crate::{
    app::{
        message::{
            success,
            AppCommand, 
            AppCommandChannel,
            GameLogicEvent,
        },
        running_flag::RunningFlag,
    }, 
    assets::bundle::AssetBundle,
    logic::{
        scene::{GameScene, NextScene},
        resource::Resources,
    },
    render::message::{
        CommandResult,
        RenderCommand,
    },
};


const MAX_UPDATE_CNT: usize = 16;
const UPDATE_FRAME_TIME: f64 = 1.0 / 60.0; // 60FPS



/// #### 한국어 </br>
/// 게임 로직 루프 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the game logic loop function. </br>
/// 
pub fn game_logic_loop(
    window: Arc<Window>,
    asset_bundle: AssetBundle,
    entry_scene: Box<dyn GameScene>,
    event_receiver: Receiver<GameLogicEvent>,
    render_cmd_sender: Sender<(Sender<CommandResult>, RenderCommand)>
) {
    // (한국어) 게임 장면에서 전역적으로 사용할 리소스를 준비합니다. 
    // (English Translation) Prepare resources for use globally in the game scene.
    let mut resources = Resources::new();
    resources.insert(window);
    resources.insert(asset_bundle);
    resources.insert(render_cmd_sender);

    // (한국어) 장면 스택을 준비합니다.
    // (English Translation) Prepare the scene stack.
    let mut scene_stack = VecDeque::from([entry_scene]);
    success(scene_stack.back_mut().unwrap().enter(&mut resources));

    log::info!("Run :: Game logic loop.");
    let mut total_time_sec = 0.0;
    'logic_loop: while let Ok(event) = event_receiver.recv() {
        // (한국어) 현재 어플리케이션이 실행중인지 확인합니다. 
        // (English Translation) Check whether the application is currently running.
        if !RunningFlag::is_running() {
            break 'logic_loop;
        }
    
        // (한국어) 스택에서 게임 장면을 가져옵니다.
        // (English Translation) Get the game scene from the stack.
        let mut scene = match scene_stack.pop_back() {
            Some(scene) => scene,
            None => {
                RunningFlag::set_exit();
                AppCommandChannel::send(AppCommand::Terminate);
                break 'logic_loop;
            }
        };

        match event {
            GameLogicEvent::NextMainEvents(elapsed_time_sec) => {
                total_time_sec += elapsed_time_sec;
            }
            GameLogicEvent::MainEventsCleared => {
                // (한국어) 일정 시간 마다 장면을 갱신합니다.
                // (English Translation) The scene is updated at regular intervals.
                let mut update_cnt = 0;
                while total_time_sec > UPDATE_FRAME_TIME && update_cnt < MAX_UPDATE_CNT {
                    update_cnt += 1;
                    total_time_sec -= UPDATE_FRAME_TIME;
                    success(scene.update(UPDATE_FRAME_TIME));
                }

                // (한국어) 성능 저하가 발생하는경우 알립니다.
                // (English Translation) Notifies if performance degradation occurs.
                if update_cnt == MAX_UPDATE_CNT {
                    log::warn!("Performance degradation occurred in the scene update section.");
                }

                // (한국어) 렌더 루프에 렌더 명령을 제출합니다.
                // (English Translation) Submit render commands to the render loop.
                success(scene.render_submit(&mut resources));
                
                // (한국어) 다음 씬을 준비합니다.
                // (English Translation) Prepare for the next scene.
                match scene.next() {
                    NextScene::Keep => { 
                        // (한국어) 현재 장면을 스택에 집어넣습니다.
                        // (English Translation) Push the current scene into the stack.
                        scene_stack.push_back(scene);
                    },
                    NextScene::Change(mut next) => {
                        // (한국어) 현재 장면을 종료합니다.
                        // (English Translation) Exit the current scene.
                        success(scene.exit(&mut resources));
                        
                        // (한국어) 새로운 장면에 들어갑니다.
                        // (English Translation) Enter the new scene.
                        success(next.enter(&mut resources));

                        // (한국어) 다음 장면을 스택에 집어넣습니다.
                        // (English Translation) Push the next scene into the stack.
                        scene_stack.push_back(next);
                    },
                    NextScene::Push(mut next) => {
                        // (한국어) 현재 장면을 스택에 집어넣습니다.
                        // (English Translation) Push the current scene into the stack.
                        scene_stack.push_back(scene);

                        // (한국어) 새로운 장면에 들어갑니다.
                        // (English Translation) Enter the new scene.
                        success(next.enter(&mut resources));

                        // (한국어) 다음 장면을 스택에 집어넣습니다.
                        // (English Translation) Push the next scene into the stack.
                        scene_stack.push_back(next);
                    },
                    NextScene::Pop => {
                        // (한국어) 현재 장면을 종료합니다.
                        // (English Translation) Exit the current scene.
                        success(scene.exit(&mut resources));
                    }
                }

                continue 'logic_loop;
            },
            _ => { 
                success(scene.handle_events(&event)); 
            }
        };

        scene_stack.push_back(scene);
    }
    log::info!("End :: Game logic loop.");
}
