/// #### 한국어 </br>
/// 에셋은 다음과 같은 에셋의 유형이 존재합니다. </br>
/// - 정적: 에셋 파일의 내용이 바뀌지 않음을 의미합니다. </br>
/// - 동적: 에셋 파일의 내용이 바뀔 수 있음을 의미합니다. </br>
/// - 선택적: 에셋 파일이 존재하지 않을 수 있고, 에셋 파일이 생성될 수 있음을 의미합니다. </br>
/// 
/// #### English (Translation)
/// There are the following asset types: </br>
/// - Static: This means that the contents of the asset file do not change. </br>
/// - Dynamic: This means that the contents of the asset file can change. </br>
/// - Optional: This means the asset file may not exist, or an asset file may be created. </br>
/// 
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Types {
    Static,
    Dynamic,
    Optional,
}

#[allow(dead_code)]
impl Types {
    /// #### 한국어 </br>
    /// 에셋이 읽기 가능한지 여부를 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// Returns whether the asset is readable </br>
    /// 
    pub fn readable(&self) -> bool {
        match self {
            Self::Static => true,
            Self::Dynamic => true,
            Self::Optional => true,
        }
    }

    /// #### 한국어 </br>
    /// 에셋이 쓰기 가능한지 여부를 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// Returns whether the asset is writable </br>
    /// 
    pub fn writable(&self) -> bool {
        match self {
            Self::Static => false,
            Self::Dynamic => true,
            Self::Optional => true,
        }
    }

    /// #### 한국어 </br>
    /// 에셋이 생성 가능한지 여부를 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// Returns whether the asset is creatable </br>
    /// 
    pub fn creatable(&self) -> bool {
        match self {
            Self::Static => false,
            Self::Dynamic => false,
            Self::Optional => true,
        }
    }
}
