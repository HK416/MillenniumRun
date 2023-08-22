use std::fmt;

use crate::{
    timer::GameTimer, 
    app::{AppResult, GameLogicEvent}
};



/// #### 한국어
/// 게임 장면의 인터페이스 입니다.
/// 
/// #### English (Translation)
/// This is the interface of the game scene.
/// 
pub trait Scene : fmt::Debug {
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
    fn update(&mut self, time: &GameTimer) -> AppResult<()>;

    /// #### 한국어
    /// 화면에 그려질 오브젝트들을 제출합니다.
    /// 
    /// #### English (Translation)
    /// Submits objects to be drawn on the screen.
    /// 
    fn submit(&self);
}
