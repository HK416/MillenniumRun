use std::fmt;

use winit::event::Event;

use crate::system::{
    error::AppResult,
    event::AppEvent,
    shared::Shared,
};



/// #### 한국어 </br>
/// 게임 장면의 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the interface of the game scene. </br>
/// 
#[allow(unused_variables)]
pub trait SceneNode : fmt::Debug {
    /// #### 한국어 </br>
    /// 게임 장면에 진입할 때 호출되는 함수입니다. </br>
    /// <b>함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// This function is called when entering the game scene. </br>
    /// <b>If an error occurs while executing the function, it returns `GameError`.</b></br>
    /// 
    #[inline]
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        Ok(())
    }

    /// #### 한국어 </br>
    /// 게임 장면을 종료할 때 호출되는 함수입니다. </br>
    /// <b>함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// This function is called when exiting the game scene. </br>
    /// <b>If an error occurs while executing the function, it returns `GameError`.</b></br>
    /// 
    #[inline]
    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        Ok(())
    }

    /// #### 한국어 </br>
    /// 애플리케이션 윈도우 이벤트를 처리하는 함수입니다. </br>
    /// <b>함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that processes application window events. </br>
    /// <b>If an error occurs while executing the function, it returns `GameError`.</b></br>
    ///  
    #[inline]
    fn handle_events(&mut self, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
        Ok(())
    }

    /// #### 한국어 </br>
    /// 게임 장면을 갱신하는 함수입니다. </br>
    /// <b>함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that updates the game scene. </br>
    /// <b>If an error occurs while executing the function, it returns `GameError`.</b></br>
    /// 
    #[inline]
    fn update(&mut self, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
        Ok(())
    }

    /// #### 한국어 </br>
    /// 게임 장면을 그리는 함수입니다. </br>
    /// <b>함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that draws the game scene. </br>
    /// <b>If an error occurs while executing the function, it returns `GameError`.</b></br>
    /// 
    #[inline]
    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        Ok(())
    }
}
