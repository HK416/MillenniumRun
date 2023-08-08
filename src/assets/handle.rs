use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak, RwLock};
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};
use super::{AssetDataType, AssetResult, AssetError, AssetErrorKind};


/// #### 한국어
/// 에셋 핸들의 내부 데이터 입니다.
/// 
/// #### English (Translation)
/// Internal data of asset handle.
/// 
#[derive(Debug)]
struct AssetHandleInner {
    /// #### 한국어
    /// 에셋 파일의 경로 입니다.
    /// 
    /// #### English (Translation)
    /// The path to the asset file.
    /// 
    path: PathBuf,

    /// #### 한국어
    /// 에셋의 파일 핸들입니다.
    /// 에셋이 로드되지 않은 경우 `None`값을 가집니다.
    /// 
    /// #### English (Translation)
    /// The asset's file handle.
    /// If the asset has not been loaded, it has the value `None`.
    /// 
    file: RwLock<Option<File>>,

    /// #### 한국어
    /// 파일 핸들의 사용 가능 유무입니다. 
    /// 파일 핸들을 사용할 수 있는 경우 `true`값을 가집니다.  
    /// 
    /// #### English (Translation)
    /// Whether the file handle is available or not. 
    /// Has a value of true if the file handle is available.  
    /// 
    available: AtomicBool,

    /// #### 한국어
    /// 에셋의 데이터 유형입니다.
    /// 
    /// #### English (Translation)
    /// The asset's data type.
    /// 
    data_type: AssetDataType,
}

impl AssetHandleInner {
    /// #### 한국어
    /// 새로운 에셋 핸들 내부 데이터를 생성합니다.
    /// 
    /// #### English (Translation)
    /// Create new asset handle internal data.
    /// 
    #[inline]
    const fn new(path: PathBuf, data_type: AssetDataType) -> Self {
        Self {
            path,
            file: RwLock::new(None),
            available: AtomicBool::new(true),
            data_type,
        }
    }

    /// #### 한국어
    /// 에셋 파일의 경로를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns the path to the asset file.
    /// 
    #[inline]
    fn file_path(&self) -> &Path {
        self.path.as_ref()
    }

    /// #### 한국어
    /// 에셋 파일 핸들을 사용할 수 있는지 여부를 반환합니다.  
    /// 
    /// #### English (Translation)
    /// Returns whether the asset file handle is available.  
    /// 
    #[inline]
    fn is_available(&self) -> bool {
        self.available.load(MemOrdering::Acquire)
    }

    /// #### 한국어
    /// 에셋 파일 핸들을 비활성화 합니다.  
    /// 
    /// #### English (Translation)
    /// Disable asset file handle.  
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
    #[inline]
    fn on_disable(&self) {
        log::warn!("handle is disabled :: <PATH:{}>, <TYPE:{:?}>", self.path.display(), self.data_type);
        self.available.store(false, MemOrdering::Release);
        *self.file.write().expect("Failed to access file handle.") = None;
    }

    /// #### 한국어
    /// 에셋 파일 핸들을 이미 가지고 있는 경우 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `true` if it already have the asset file handle. 
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 내부 뮤텍스 잠금 중에 오류가 발생할 경우 프로그램 실행을 중단시킵니다. 
    /// 자세한 내용은 [`std::sync::RwLock::read`](`std::sync::RwLock`)문서를 참고하세요.
    /// 
    /// #### English (Translation)
    /// Abort program execution if an error occurs while locking an internal mutex. 
    /// See the [`std::sync::RwLock::read`](`std::sync::RwLock`) documentation for details.
    /// 
    #[inline]
    fn has_loaded(&self) -> bool {
        self.file.read().expect("Failed to access file handle.").is_some()
    }

    /// #### 한국어
    /// 에셋 파일을 에셋의 데이터 유형에따라 다른 모드로 엽니다.
    /// - 정적 데이터 타입: 읽기 모드로 엽니다.
    /// - 동적 데이터 타입: 읽기/쓰기 모드로 엽니다.
    /// - 선택적 데이터 타입: 읽기/쓰기/생성 모드로 엽니다.
    /// 
    /// #### English (Translation)
    /// Open the asset file in different modes depending on the asset's data type.
    /// - Static data type: open in read mode.
    /// - Dynamic data type: open in read/write mode.
    /// - Optional data type: open in read/write/create mode.
    /// 
    /// <br>
    /// 
    /// # Errors
    /// #### 한국어
    /// 다음과 같은 경우 `AssetErrorKind`를 반환합니다.
    /// - 에셋 핸들이 비활성화되었을 경우 `AssetErrorKind::DisabledHandle`을 반환합니다.
    /// - 에셋 파일을 여는 도중 오류가 발생하는 경우.
    /// 
    /// #### English (Translation)
    /// It returns `AssetErrorKind` for the following cases:
    /// - Return `AssetErrorKind::DisabledHandle` if the asset handle is disabled.
    /// - If an error occurs while opening the asset file.
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
    fn load(&self) -> AssetResult<()> {
        log::debug!("load asset handle :: <PATH:{}> <TYPE:{:?}>", self.path.display(), self.data_type);

        if !self.is_available() { 
            return Err(AssetError::new(
                AssetErrorKind::DisabledHandle, 
                format!("Asset handle is disabled.")
            )); 
        }

        if !self.has_loaded() { 
            *self.file.write().expect("Failed to access file handle.") = Some(
                match self.data_type {
                    AssetDataType::StaticData => OpenOptions::new()
                        .read(true)
                        .open(&self.path),
                    AssetDataType::DynamicData => OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(&self.path),
                    AssetDataType::OptionalData => OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&self.path),
                }.map_err(|err| AssetError::new(
                    AssetErrorKind::from(err),
                    format!("Failed to open asset file.")
                ))?
            );
        }

        Ok(())
    }
}



/// #### 한국어
/// 에셋 파일의 핸들입니다. 핸들을 복제하여 여러 스레드에서 공유하여 사용할 수 있습니다.
/// 핸들을 복제할 때 내부 참조 횟수를 증가시키고 싶지 않을 경우 `AssetHandle::downgrade`를 사용해야 합니다.  
/// 
/// #### English (Translation)
/// The handle of the asset file. By duplicating the handle, it can be shared and used by multiple threads. 
/// You should use `AssetHandle::downgrade` if you do not want to increment the internal reference count when duplicating handles.  
/// 
#[derive(Debug, Clone)]
pub struct AssetHandle(Arc<AssetHandleInner>);

impl AssetHandle {
    /// #### 한국어
    /// 에셋 설명자로부터 로드되지 않은 에셋 핸들을 생성하는 함수입니다.
    /// 
    /// #### English (Translation)
    /// A function that creates an unloaded asset handle from an asset descriptor.
    /// 
    #[inline]
    pub(super) fn new(path: PathBuf, data_type: AssetDataType) -> Self {
        Self(Arc::new(AssetHandleInner::new(path, data_type)))
    }

    /// #### 한국어
    /// 에셋 파일의 핸들을 생성합니다.
    /// 
    /// #### English (Translation)
    /// Creates a handle to an asset file.
    /// 
    /// <br>
    /// 
    /// # Errors
    /// #### 한국어
    /// 다음과 같은 경우 `AssetErrorKind`를 반환합니다.
    /// - 에셋 핸들이 비활성화되었을 경우 `AssetErrorKind::DisabledHandle`을 반환합니다.
    /// - 에셋 파일의 핸들이 이미 존재하는 경우 `AssetErrorKind::AlreadyLoaded`를 반환합니다.
    /// - 에셋 파일을 여는 도중 오류가 발생하는 경우.
    /// 
    /// #### English (Translation)
    /// It returns `AssetErrorKind` for the following cases:
    /// - Return `AssetErrorKind::DisabledHandle` if the asset handle is disabled.
    /// - Return `AssetErrorKind::AlreadyLoaded` if the asset file's handle already exists.
    /// - If an error occurs while opening the asset file.
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
    #[inline]
    pub(super) fn load_asset(&self) -> AssetResult<()> {
        self.0.load()
    }

    /// #### 한국어
    /// 핸들을 비활성화 시킵니다.  
    /// 
    /// #### English (Translation)
    /// Disable the handle.  
    /// 
    #[inline]
    pub(super) fn on_disable(&self) {
        self.0.on_disable()
    }

    /// #### 한국어
    /// 에셋 핸들을 사용할 수 있는지 여부를 반환합니다.  
    /// 
    /// #### English (Translation)
    /// Returns whether the asset handle is available.  
    /// 
    #[inline]
    pub fn is_available(&self) -> bool {
        self.0.is_available()
    }
    
    /// #### 한국어
    /// 내부 참조 횟수를 증가시키지 않고 에셋 파일 핸들을 복제합니다.  
    /// 
    /// #### English (Translation)
    /// Duplicate the asset file handle without incrementing the internal reference count.  
    /// 
    #[inline]
    pub fn downgrade(ptr: &Self) -> WeakAssetHandle {
        WeakAssetHandle(Arc::downgrade(&ptr.0))
    }
}


/// #### 한국어
/// 에셋 파일의 핸들입니다. 핸들을 복제하여 여러 스레드에서 공유하여 사용할 수 있습니다.
/// 이 에셋 파일 핸들은 복제할 때 내부 참조 횟수를 증가시키지 않습니다. 따라서 원본 에셋 파일 핸들이 삭제된 상태일 수 있습니다. 
/// 에셋 파일 핸들의 기능을 사용하려면 `WeakAssetHandle::upgrade`함수를 사용해 `AssetHandle`로 업그레이드 해야 합니다.  
/// 
/// #### English (Translation)
/// A handle to the asset file. By duplicating the handle, it can be shared and used by multiple threads.
/// This asset file handle does not increment its internal reference count when cloned. Therefore, the original asset file handle may be in a deleted state.
/// To use the functionality of asset file handles, you need to upgrade them to `AssetHandle` using `WeakAssetHandle::upgrade` function.  
/// 
#[derive(Debug, Clone)]
pub struct WeakAssetHandle(Weak<AssetHandleInner>);

impl WeakAssetHandle {
    /// ### 한국어
    /// `AssetHandle`로 업그레이드하려고 시도합니다.  
    /// 원본 `AssetHandle`이 이미 삭제된 경우 `None`을 반환합니다.  
    /// 
    /// ### English (Translation)
    /// Attempting to upgrade to `AssetHandle`.  
    /// Returns 'None' if the original 'AssetHandle' has already been deleted.  
    /// 
    #[inline]
    pub fn upgrade(&self) -> Option<AssetHandle> {
        Some(AssetHandle(self.0.upgrade()?))
    }
}
