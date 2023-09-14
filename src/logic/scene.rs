use std::fmt;

use crate::{
    app::{
        abort::AppResult,
        message::GameLogicEvent,
    },
    logic::resource::Resources,
};



/// #### 한국어 </br>
/// 게임 장면의 다음 장면 정보 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the next scene information from the game scene. </br>
/// 
#[derive(Debug)]
pub enum NextScene {
    Keep,
    Change(Box<dyn GameScene>),
    Push(Box<dyn GameScene>),
    Pop,
}


/// #### 한국어
/// 게임 장면의 인터페이스 입니다.
/// 
/// #### English (Translation)
/// This is the interface of the game scene.
/// 
pub trait GameScene : fmt::Debug {
    /// #### 한국어
    /// 장면에 진입할 때 호출되는 함수입니다. </br>
    /// 장면의 구성요소를 생성하거나 초기화 합니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when entering the scene. </br>
    /// Creates or initializes scene components. </br>
    /// 
    /// <br>
    /// 
    /// # Note
    /// #### 한국어
    /// 개발자는 함수 내에서 `panic!`을 호출하는 것을 자제해야 합니다. 
    /// 대신 오류가 발생한 경우 `AppResult`로 오류를 반환해야 합니다.
    /// 
    /// #### English (Translation)
    /// Developers should refrain from calling `panic!` within a function.
    /// Instead, Developers should return an error as `AppResult` if an error occurs.
    /// 
    fn enter(&mut self, res: &mut Resources) -> AppResult<()>;

    /// #### 한국어
    /// 장면을 빠져나갈 때 호출되는 함수입니다. </br>
    /// 장면의 구성요소를 저장하거나 정리합니다. </br>
    /// 
    /// #### English (Translation)
    /// This function is called when exting the scene. </br>
    /// Save or clean up the scene components. </br>
    /// 
    /// <br>
    /// 
    /// # Note
    /// #### 한국어
    /// 개발자는 함수 내에서 `panic!`을 호출하는 것을 자제해야 합니다. 
    /// 대신 오류가 발생한 경우 `AppResult`로 오류를 반환해야 합니다.
    /// 
    /// #### English (Translation)
    /// Developers should refrain from calling `panic!` within a function.
    /// Instead, Developers should return an error as `AppResult` if an error occurs.
    /// 
    fn exit(&mut self, res: &mut Resources) -> AppResult<()>;

    /// #### 한국어
    /// 시스템 및 윈도우 이벤트를 처리하는 함수입니다.
    /// 
    /// #### English (Translation)
    /// A function that handles system and window events.
    /// 
    /// <br>
    /// 
    /// # Note
    /// #### 한국어
    /// 개발자는 함수 내에서 `panic!`을 호출하는 것을 자제해야 합니다. 
    /// 대신 오류가 발생한 경우 `AppResult`로 오류를 반환해야 합니다.
    /// 
    /// #### English (Translation)
    /// Developers should refrain from calling `panic!` within a function.
    /// Instead, Developers should return an error as `AppResult` if an error occurs.
    /// 
    fn handle_events(&mut self, event: &GameLogicEvent) -> AppResult<()>;
    
    /// #### 한국어
    /// 주어진 시간 만큼 장면을 갱신하는 함수입니다.
    /// 
    /// #### English (Translation)
    /// This function updates the scene for a given elapsed time.
    /// 
    /// <br>
    /// 
    /// # Note
    /// #### 한국어
    /// 개발자는 함수 내에서 `panic!`을 호출하는 것을 자제해야 합니다. 
    /// 대신 오류가 발생한 경우 `AppResult`로 오류를 반환해야 합니다.
    /// 
    /// #### English (Translation)
    /// Developers should refrain from calling `panic!` within a function.
    /// Instead, Developers should return an error as `AppResult` if an error occurs.
    /// 
    fn update(&mut self, time: f64) -> AppResult<()>;

    /// #### 한국어
    /// 화면에 그려질 오브젝트들을 제출합니다.
    /// 
    /// #### English (Translation)
    /// Submits objects to be drawn on the screen.
    /// 
    fn render_submit(&self, res: &mut Resources) -> AppResult<()>;

    /// #### 한국어
    /// 다음 장면으로 이동합니다.
    fn next(&self) -> NextScene;
}
