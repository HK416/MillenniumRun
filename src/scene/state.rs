use crate::scene::node::SceneNode;



/// #### 한국어 </br>
/// 게임 장면의 다음 장면 상태변화를 나타냅니다. </br>
/// 
/// #### English (Translation) </br>
/// Indicates the next scene state change in the game scene. </br>
/// 
#[derive(Debug, Default)]
pub enum SceneState {
    /// #### 한국어 </br>
    /// 현재 장면을 유지합니다. 이 값은 기본값 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Retains the current scene. This value is the default. </br>
    /// 
    #[default]
    Keep,

    /// #### 한국어 </br>
    /// 현재 장면을 종료합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Exit the current scene. </br>
    /// 
    Pop,

    /// #### 한국어 </br>
    /// 새로운 장면에 진입합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Enter the new scene. </br>
    /// 
    Push(Box<dyn SceneNode>),
    
    /// #### 한국어 </br>
    /// 현재 장면을 종료하고, 새로운 장면에 진입합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Exit the current scene and enter the new scene. </br>
    /// 
    Change(Box<dyn SceneNode>),

    /// #### 한국어 </br>
    /// 모든 장면을 종료하고, 새로운 장면에 진입합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Eixt the all scene and enter the new scene. </br>
    /// 
    Reset(Box<dyn SceneNode>),
}
