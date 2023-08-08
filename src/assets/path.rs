use std::env;
use std::fmt;
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use super::{AssetResult, AssetErrorKind, AssetError};


const ASSETS_REL_PATH: &'static str = "assets";

lazy_static! {
    static ref ASSETS_PATH: AssetResult<PathBuf> = {
        let path = PathBuf::from_iter([
            env::current_exe()
                .map_err(|e| AssetError::new(
                    AssetErrorKind::from(e),
                    format!("Failed to get file path of current application.")
                ))?
                .parent()
                .map_or_else(
                    || Err(AssetError::new(
                        AssetErrorKind::NotFound,
                        format!("Failed to get directory path of current application.")
                    )), 
                    |path| Ok(path)
                )?,
            Path::new(ASSETS_REL_PATH),
        ])
        .canonicalize()
        .map_err(|e| AssetError::new(
            AssetErrorKind::from(e),
            format!("The path to the assets directory could not be found!")
        ))?;
    
        match path.is_dir() {
            true => Ok(path),
            false => Err(AssetError::new(
                AssetErrorKind::NotDirectory,
                format!("The path to the assets directory could not be found!"),
            )),
        }
    };
}



/// #### 한국어
/// 에셋 디렉토리의 최상위 경로 입니다.
/// 에셋 디렉토리의 경로는 프로그램이 시작될 때 생성됩니다.
/// 
/// #### English (Translation)
/// The top-level path of the assets directory.
/// The path to the assets directory is created when the program starts.
/// 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootAssetPath(PathBuf);

impl RootAssetPath {
    /// #### 한국어
    /// 에셋 디렉토리의 경로를 가져옵니다.
    /// 만약 에셋 디렉토리를 찾지 못했을 경우 `AssetError`를 반환합니다.  </br>
    /// 에셋 디렉토리의 최상위 경로는 프로그램이 시작될 때 생성되기 때문에 
    /// `RootAssetPath::check_asset_path`를 호출후 `RootAssetPath::get_unchecked`를 사용하는 것을 추천합니다.
    /// 
    /// #### English
    /// Get the path to the assets directory.
    /// Return `AssetError` if the asset directory is not found. </br>
    /// Since the top-level path of the asset directory is created when the program starts, 
    /// it is recommended to use `RootAssetPath::get_unchecked` after calling `RootAssetPath::check_asset_path`.
    /// 
    #[inline]
    pub fn get() -> AssetResult<Self> {
        Ok(Self(ASSETS_PATH.clone()?))
    }

    /// #### 한국어
    /// 에셋 디렉토리의 경로를 가져옵니다.
    /// 이 함수는 에셋 디렉토리 경로를 확인하지 않습니다.
    /// 
    /// #### English (Translation)
    /// Get the path to the assets directory.
    /// This function does not check the asset directory path.
    /// 
    /// <br>
    /// 
    /// # Safety
    /// #### 한국어
    /// 이 함수를 호출하기 이전에 `RootAssetPath::check_asset_path`를 호출하여 오류가 있는지 확인하십시오.
    /// 
    /// #### English (Translation)
    /// Before calling this function, check for errors by calling `RootAssetPath::check_asset_path`.
    /// 
    #[inline]
    pub unsafe fn get_unchecked() -> Self {
        Self(ASSETS_PATH.clone().unwrap_unchecked())
    }

    /// #### 한국어
    /// 에셋 디렉토리 경로에 오류가 발생했는지 확인합니다.
    /// 오류가 발생했을 경우 `AssetError`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Checking the asset directory path for errors.
    /// Return `AssetError` if an error occurs.
    /// 
    #[inline]
    pub fn check_asset_path() -> Option<AssetError> {
        ASSETS_PATH.clone().err()
    }
}

impl AsRef<Path> for RootAssetPath {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl fmt::Display for RootAssetPath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
