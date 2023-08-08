use std::io;
use std::fs;
use std::thread; 
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use notify::Result as NotifyResult;

use super::{
    AssetHandle,
    AssetKeys,
    AssetCache,
    AssetResult,
    AssetError,
    AssetErrorKind,
    watcher_main,
    RootAssetPath, 
    AssetDataType,
    ASSET_LIST,
};



/// #### 한국어
/// 어플리케이션에서 사용하는 모든 에셋을 관리합니다.
/// `AssetBundle`을 복제하여 여러 스레드에서 공유하여 사용할 수 있습니다.
/// 
/// #### English (Translation)
/// Manage all assets used by the application.
/// By duplicating the `AssetBundle`, it can be shared and used by multiple threads.
/// 
#[derive(Debug, Clone)]
pub struct AssetBundle {
    /// #### 한국어
    /// 에셋 디렉토리 경로 입니다.
    /// 프로그램 파일의 위치에 따라 다른 절대 경로를 가집니다.
    /// 
    /// #### English (Translation)
    /// Asset directory path.
    /// It has a different absolute path depending on where the program files are located.
    /// 
    root_path: RootAssetPath,

    /// #### 한국어
    /// 미리 수집한 에셋을 관리합니다.
    /// 어플리케이션은 미리 수집한 에셋만을 사용할 수 있습니다.
    /// `AssetLists.txt`에서 에셋을 추가하거나 제거할 수 있습니다.
    /// 
    /// #### English (Translation)
    /// Manage pre-collected assets.
    /// Application can only use pre-collected assets.
    /// Developer can add or remove assets from `AssetLists.txt`.
    /// 
    asset_cache: AssetCache,
}

impl AssetBundle {
    /// #### 한국어
    /// 새로운 `AssetBundle`을 생성합니다.
    /// 
    /// #### English (Translation)
    /// Creates a new `AssetBundle`.
    /// 
    /// <br>
    /// 
    /// # Errors
    /// #### 한국어
    /// 에셋 디렉토리 경로를 찾을 수 없거나, 에셋 파일이 손상되었을 경우 `AssetError`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns `AssetError` if the asset directory path is not found or if the asset file is corrupted.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 내부 뮤텍스 잠금 중에 오류가 발생할 경우 프로그램 실행을 중단시킵니다. 
    /// 자세한 내용은 [`std::sync::RwLock::write`](`std::sync::RwLock`)문서를 참고하세요.  
    /// 
    /// #### English (Translation)
    /// Abort program execution if an error occurs while locking an internal mutex. 
    /// See the [`std::sync::RwLock::write`](`std::sync::RwLock`) documentation for details.  
    /// 
    pub fn new() -> AssetResult<Self> {
        if let Some(err) = RootAssetPath::check_asset_path() {
            return Err(err);
        }

        // (한국어) 안전: 이전에 경로가 존재하는지 확인했음.
        // (English Translation) SAFETY: previously checked that the path exists.
        let root_path = unsafe { RootAssetPath::get_unchecked() };
        log::info!("Found the assets directory. (PATH: {})", root_path);

        let asset_cache = AssetCache::new();
        let root_path_cloned = root_path.clone();
        let asset_cache_cloned = asset_cache.clone();
        thread::spawn(move || -> NotifyResult<()> {
            watcher_main(root_path_cloned, asset_cache_cloned)
        });

        let asset_list = check_assets(&root_path)?;
        asset_cache.overwrite_handle_batch(asset_list)?;

        Ok(Self { root_path, asset_cache })
    }

    /// #### 한국어
    /// 주어진 경로에 캐싱되어 있는 에셋을 불러옵니다.
    /// 이 함수는 현재 스레드를 차단하지 않습니다.
    /// 
    /// #### English (Translation)
    /// Loads the cached assets in the given path.
    /// This function does not block the current thread.
    /// 
    /// <br>
    /// 
    /// # Errors
    /// #### 한국어
    /// 주어진 경로에 캐싱되어 있는 에셋이 존재하지 않거나, 에셋을 로드할때 오류가 발생한 경우 `AssetError`를 반환합니다.
    /// 
    /// #### English
    /// If an asset cached in the given path does not exist or an error occurs when loading an asset, `AssetError` is returned.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 내부 뮤텍스 잠금 중에 오류가 발생할 경우 프로그램 실행을 중단시킵니다. 
    /// 자세한 내용은 [`std::sync::RwLock::write`](`std::sync::RwLock`)문서를 참고하세요.  
    /// 
    /// #### English (Translation)
    /// Abort program execution if an error occurs while locking an internal mutex. 
    /// See the [`std::sync::RwLock::write`](`std::sync::RwLock`) documentation for details.  
    /// 
    pub async fn load_asset<P: AsRef<Path>>(&self, path: P) -> AssetResult<AssetHandle> {
        let path = PathBuf::from_iter([self.root_path.as_ref(), path.as_ref()]);
        log::debug!("load assets. (PATH:{})", path.display());
        if let Some(handle) = self.asset_cache.get_handle(path) {
            handle.load_asset()?;
            return Ok(handle);
        }

        return Err(AssetError::new(
            AssetErrorKind::NotFound, format!("Asset not found.")
        ));
    }
}


/// #### 한국어
/// `AssetLists.txt`에 주어진 에셋을 확인합니다.
/// `AssetLists.txt`목록의 에셋은 컴파일 시간에 파일이 존재하는지 확인 후 바이너리에 포함됩니다.
/// 모든 정적 및 동적 데이터 파일이 존재하는지 검사하고, 정적 데이터에 대해 파일 무결성을 검사합니다.
/// 
/// #### English
/// Check the assets given in `AssetLists.txt`
/// Assets in the `AssetLists.txt` list are included in the binary after checking if the file exists at compile time.
/// All static and dynamic data files are checked for existence, and file integrity is checked for static data.
/// 
/// <br>
/// 
/// # Errors
/// #### 한국어
/// 에셋의 경로를 찾을 수 없거나, 파일이 손상된 경우 `AssetError`를 반환합니다.
/// 
/// #### English (Translation)
/// Return `AssetError` if the path to the asset cannot be found or the file is corrupt.
/// 
fn check_assets<P: AsRef<Path>>(root_path: P) -> AssetResult<HashMap<PathBuf, AssetDataType>> {
    let mut asset_list = HashMap::with_capacity(ASSET_LIST.capacity());
    
    for (path, data_type) in ASSET_LIST.iter() {
        let asset_path = PathBuf::from_iter([root_path.as_ref(), &path]);
        if !data_type.is_optional() && !asset_path.is_file() {
            return Err(AssetError::new(
                AssetErrorKind::NotFile,
                format!("Asset is not a file or path cannot be resolved!")
            ));
        }

        if data_type.is_static_data() {
            let embed_file = AssetKeys::get(path.to_str().unwrap()).ok_or_else(
                || AssetError::new(
                    AssetErrorKind::NotFound, 
                    format!("Key value not found!")
                )
            )?;

            let hash = {
                let mut file = fs::File::open(&asset_path)
                    .map_err(|e| AssetError::new(
                        AssetErrorKind::from(e), format!("Unable to open asset file.")
                    ))?;
                let mut hasher = Sha256::new();
                io::copy(&mut file, &mut hasher)
                    .map_err(|e| AssetError::new(
                        AssetErrorKind::from(e), format!("Key value creation failed.")
                    ))?;
                hasher.finalize()
            };

            if embed_file.data.as_ref().ne(hash.as_slice()) {
                return Err(AssetError::new(
                    AssetErrorKind::InvalidKey,
                    format!("The application's asset file is corrupted."),
                ));
            }
        }
        asset_list.insert(asset_path, *data_type);
    }

    return Ok(asset_list)
}
