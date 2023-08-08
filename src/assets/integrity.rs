use rust_embed::RustEmbed;

/// #### 한국어
/// 에셋의 데이터 유형입니다.
/// 데이터 유형은 정적 데이터와 동적 데이터 그리고 선택적 데이터 세 가지가 있습니다.
/// 정적 데이터 유형은 프로그램 실행 도중 데이터의 내용이 바뀌지 않음을 의미합니다.
/// 동적 데이터 유형은 프로그램 실행 도중 데이터의 내용이 바뀔 수 있음을 의미합니다.
/// 선택적 데이터 유형은 프로그램 실행 중에 사용될 수 있거나 사용되지 않을 수 있음을 의미합니다.
/// 또한 선택적 데이터 유형은 기본적으로 동적 데이터 유형처럼 데이터의 내용이 바뀔 수 있음을 의미합니다.
/// 기본 값은 정적 데이터 유형입니다.
/// 
/// #### English (Translation)
/// The asset's data type.
/// There are three types of data: static data and dynamic data, and optional data.
/// A static data type means that the contents of the data do not change during program execution.
/// A dynamic data type means that the contents of the data can change during program execution.
/// An optional data type means that it may or may not be used during program execution.
/// Optional data types also basically mean that the contents of the data can change, just like the dynamic data type.
/// The default value is a static data type.
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetDataType {
    #[default]
    StaticData,
    DynamicData,
    OptionalData,
}

impl AssetDataType {
    /// #### 한국어
    /// 에셋 데이터가 정적인 경우에 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `true` if the asset data is static.
    /// 
    #[inline]
    pub fn is_static_data(self) -> bool {
        match self {
            AssetDataType::StaticData => true,
            AssetDataType::DynamicData => false,
            AssetDataType::OptionalData => false,
        }
    }
    
    /// #### 한국어
    /// 에셋 데이터가 동적인 경우에 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `true` if the asset data is dynamic.
    /// 
    #[inline]
    pub fn is_dynamic_data(self) -> bool {
        match self {
            AssetDataType::StaticData => false,
            AssetDataType::DynamicData => true,
            AssetDataType::OptionalData => true,
        }
    }

    /// #### 한국어
    /// 에셋 데이터가 선택적인 경우에 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `true` if the asset data is optional.
    /// 
    #[inline]
    pub fn is_optional(self) -> bool {
        match self {
            AssetDataType::StaticData => false,
            AssetDataType::DynamicData => false,
            AssetDataType::OptionalData => true,
        }
    }
}


#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/keys/"]
pub struct AssetKeys;
