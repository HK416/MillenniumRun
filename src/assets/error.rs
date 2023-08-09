use std::io;

/// #### 한국어
/// `AssetError`를 반환하는 [`std::result::Result`]의 래퍼 타입입니다.
/// 
/// #### English (Translation)
/// A wrapper type for [`std::result::Result`] that returns `AssetError`.
/// 
pub type AssetResult<T> = Result<T, AssetError>;


/// #### 한국어
/// 에셋에서 오류가 발생했을 경우 반환되는 오류 타입입니다.
/// 
/// #### English (Translation)
/// This is the error type returned when an error occurs in an asset.
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetError {
    /// #### 한국어
    /// 에셋 에러의 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of asset error.
    /// 
    kind: AssetErrorKind,

    /// #### 한국어
    /// 사용자에게 전달할 에러 메시지 입니다.
    /// 
    /// #### English (Translation)
    /// This is the error message to be delivered to the user.
    /// 
    msg: String,
}

impl AssetError {
    #[inline]
    pub const fn new(kind: AssetErrorKind, msg: String) -> Self {
        Self { kind, msg }
    }

    #[inline]
    pub fn kind(&self) -> &AssetErrorKind {
        &self.kind
    }

    #[inline]
    pub fn msg(&self) -> &str {
        &self.msg
    }
}


/// #### 한국어
/// 에셋을 관리하면서 발생 할 수 있는 오류의 유형입니다.
/// 
/// #### English (Translation)
/// These are the types of errors that can occur while managing assets.
/// 
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum AssetErrorKind {
    /// #### 한국어
    /// 알 수 없는 유형의 오류에 대한 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The error type for errors of unknown type.
    /// 
    #[default]
    Unknown,

    /// #### 한국어
    /// 비활성화된 에셋 핸들을 사용하려 할때 반환되는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of error returned when trying to use a disabled asset handle.
    /// 
    DisabledHandle,

    /// #### 한국어
    /// 에셋의 키값이 같지 않을 경우 반환되는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of error returned if the asset's key values are not equal.
    /// 
    InvalidKey,

    /// #### 한국어
    /// 에셋 핸들의 캐시가 안전하지 않은 경우 반환되는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of error returned if the asset handle cache is not safe.
    /// 
    UnsafetyCache,

    /// #### 한국어
    /// 찾지 못한 경우 반환되는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of error returned if the it was not found.
    /// 
    NotFound,

    /// #### 한국어
    /// 경로가 디렉토리가 아닌 경우 반환되는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of error returned if the path is not a directory.
    /// 
    NotDirectory,

    /// #### 한국어
    /// 경로가 에셋 파일이 아닌 경우 반환되는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The type of error returned if the path is not a asset file.
    /// 
    NotFile,

    /// #### 한국어
    /// [`std::io::ErrorKind::AlreadyExists`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`std::io::ErrorKind::AlreadyExists`].
    /// 
    AlreadyExists,

    /// #### 한국어
    /// [`std::io::ErrorKind::PermissionDenied`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`std::io::ErrorKind::PermissionDenied`].
    /// 
    PermissionDenied,

    /// #### 한국어
    /// [`std::io::ErrorKind::Interrupted`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`std::io::ErrorKind::Interrupted`].
    /// 
    Interrupted,

    /// #### 한국어
    /// [`std::io::ErrorKind::Unsupported`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`std::io::ErrorKind::Unsupported`].
    /// 
    UnsupportedPlatform,

    /// #### 한국어
    /// [`std::io::ErrorKind::UnexpectedEof`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`std::io::ErrorKind::UnexpectedEof`].
    /// 
    UnexpectedEOF,

    /// #### 한국어
    /// [`std::io::ErrorKind::OutOfMemory`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`std::io::ErrorKind::OutOfMemory`].
    /// 
    OutOfMemory,

    /// #### 한국어
    /// [`std::io::ErrorKind`]의 나머지 유형에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// The error type corresponding to the remaining types in [`std::io::ErrorKind`].
    /// 
    IOOtherError(io::ErrorKind),

    /// #### 한국어
    /// [`ron::Error`]에 해당하는 오류 유형입니다.
    /// 
    /// #### English (Translation)
    /// An error type that corresponds to [`ron::Error`].
    /// 
    RonParsingError(ron::Error),
}

impl From<io::Error> for AssetErrorKind {
    #[inline]
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => Self::NotFound,
            io::ErrorKind::AlreadyExists => Self::AlreadyExists,
            io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            io::ErrorKind::Interrupted => Self::Interrupted,
            io::ErrorKind::Unsupported => Self::UnsupportedPlatform,
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            io::ErrorKind::OutOfMemory => Self::OutOfMemory,
            _ => Self::IOOtherError(error.kind())
        }
    }
}

impl From<ron::Error> for AssetErrorKind {
    #[inline]
    fn from(error: ron::Error) -> Self {
        Self::RonParsingError(error)
    }
}
