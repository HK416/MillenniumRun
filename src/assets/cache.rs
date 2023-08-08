use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};
use std::collections::HashMap;

use super::{
    AssetHandle, 
    AssetResult, 
    AssetError, 
    AssetErrorKind, 
    AssetDataType
};



/// #### 한국어
/// 어플리케이션에서 관리하는 에셋의 핸들을 저장합니다.
/// 
/// #### English (Translation)
/// Stores handles of assets managed by the application.
/// 
#[derive(Debug, Clone)]
pub struct AssetCache {
    /// #### 한국어
    /// 캐시된 에셋 핸들 입니다.
    /// 
    /// #### English (Translation)
    /// A cached asset handles.
    /// 
    handles: Arc<RwLock<HashMap<PathBuf, AssetHandle>>>,

    /// #### 한국어
    /// 파일 무결성 확인을 위한 플래그 변수 입니다.
    /// `false`일 경우 외부에서 파일 변조를 시도한 것으로 간주합니다.
    /// 
    /// #### English (Translation)
    /// Flag variable for file integrity check.
    /// If it is `false`, it is regarded as an attempt to tamper with the file from outside.
    /// 
    safety: Arc<AtomicBool>,
}

impl AssetCache {
    /// #### 한국어
    /// 비어있는 `AssetCache`를 생성합니다.
    /// 
    /// #### English (Translation)
    /// Create an empty `AssetCache`.
    /// 
    #[inline]
    pub fn new() -> Self {
        Self { 
            handles: Arc::new(RwLock::new(HashMap::new())), 
            safety: Arc::new(AtomicBool::new(true)),
        }
    }

    /// #### 한국어
    /// 해당 경로의 에셋 핸들을 새로운 핸들로 덮어씁니다.  
    /// 해당 경로에 기존 핸들이 존재할 경우 기존 핸들은 비활성화 됩니다. 
    /// 
    /// #### English (Translation)
    /// Overwrites the asset handle at that path with the new handle.  
    /// If an existing handle exists in the path, the existing handle is disabled.  
    /// 
    /// # Errors
    /// #### 한국어
    /// 현재 에셋 핸들 캐시가 안전하지 않은 상태일 경우 `AssetErrorKind::UnsafetyCache`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `AssetErrorKind::UnsafetyCache` if the current asset handle cache is in an unsafe state.
    /// 
    /// # Panics
    /// #### 한국어
    /// 내부 뮤텍스 잠금 중에 오류가 발생할 경우 프로그램 실행을 중단시킵니다. 
    /// 자세한 내용은 [`std::sync::RwLock`] 문서를 참고하세요.  
    /// 
    /// #### English (Translation)
    /// Abort program execution if an error occurs while locking an internal mutex. 
    /// See the [`std::sync::RwLock`] documentation for details.  
    /// 
    pub fn overwrite_handle(&self, item: (PathBuf, AssetDataType)) -> AssetResult<()> {
        if !self.is_safety() { 
            return Err(AssetError::new(
                AssetErrorKind::UnsafetyCache, format!("Asset file integrity is corrupted.")
            )); 
        }
        
        let (path, data_type) = item.into();
        let mut handles = self.handles.write().expect("Failed to access asset cache.");
        log::debug!("cached in asset handle :: <PATH:{}> <TYPE:{:?}>", path.display(), data_type);
        if let Some(old_handle) = handles.insert(
            path.clone(), 
            AssetHandle::new(path, data_type)
        ) {
            old_handle.on_disable();
        }
        Ok(())
    }

    /// #### 한국어
    /// 해당 경로의 에셋 핸들을 새로운 핸들로 덮어씁니다.  
    /// 해당 경로에 기존 핸들이 존재할 경우 기존 핸들은 비활성화 됩니다. 
    /// 
    /// #### English (Translation)
    /// Overwrites the asset handle at that path with the new handle.  
    /// If an existing handle exists in the path, the existing handle is disabled.  
    /// 
    /// # Errors
    /// #### 한국어
    /// 현재 에셋 핸들 캐시가 안전하지 않은 상태일 경우 `AssetErrorKind::UnsafetyCache`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `AssetErrorKind::UnsafetyCache` if the current asset handle cache is in an unsafe state.
    /// 
    /// # Panics
    /// #### 한국어
    /// 내부 뮤텍스 잠금 중에 오류가 발생할 경우 프로그램 실행을 중단시킵니다. 
    /// 자세한 내용은 [`std::sync::RwLock`] 문서를 참고하세요.  
    /// 
    /// #### English (Translation)
    /// Abort program execution if an error occurs while locking an internal mutex. 
    /// See the [`std::sync::RwLock`] documentation for details.  
    /// 
    #[inline]
    pub fn overwrite_handle_batch<I>(&self, items: I) -> AssetResult<()>
    where 
        I: IntoIterator<Item = (PathBuf, AssetDataType)>,
        I::IntoIter: ExactSizeIterator,
    {
        if !self.is_safety() { 
            return Err(AssetError::new(
                AssetErrorKind::UnsafetyCache, format!("Asset file integrity is corrupted.")
            )); 
        }

        let mut handles = self.handles.write().expect("Failed to access asset cache.");
        for (path, data_type) in items.into_iter() {
            log::debug!("cached in asset handle :: <PATH:{}> <TYPE:{:?}>", path.display(), data_type);
            if let Some(old_handle) = handles.insert(
                path.clone(), 
                AssetHandle::new(path, data_type)
            ) {
                old_handle.on_disable();
            }
        }
        Ok(())
    }

    /// #### 한국어
    /// 캐시에서 주어진 경로의 에셋 핸들을 가져옵니다. 
    /// 주어진 경로의 에셋 핸들이 없는 경우 `None`을 반환합니다. 
    /// 
    /// #### English (Translation)
    /// Gets the asset handle of the given path from the cache.  
    /// Returns `None` if there is no asset handle for the given path.  
    /// 
    /// # Panics
    /// #### 한국어
    /// 내부 뮤텍스 잠금 중에 오류가 발생할 경우 프로그램 실행을 중단시킵니다. 
    /// 자세한 내용은 [`std::sync::RwLock`] 문서를 참고하세요.  
    /// 
    /// #### English (Translation)
    /// Abort program execution if an error occurs while locking an internal mutex. 
    /// See the [`std::sync::RwLock`] documentation for details.  
    /// 
    #[inline]
    pub fn get_handle<P: AsRef<Path>>(&self, path: P) -> Option<AssetHandle> {
        self.handles.read().expect("Failed to access asset cache.")
            .get(path.as_ref())
            .cloned()
    }

    /// #### 한국어
    /// 캐시된 에셋 핸들의 데이터에 이상이 발생한 경우 호출합니다.
    /// 모든 에셋 핸들을 비활성화 처리 합니다.
    /// 
    /// #### English (Translation)
    /// Called when an error occurs with the cached asset handle data.
    /// Disable all asset handles.
    /// 
    #[inline]
    pub fn on_unsafety(&self) {
        log::warn!("asset cache is disabled.");
        self.safety.store(false, MemOrdering::Release);
        let mut handles = self.handles.write().expect("Failed to access asset cache.");
        for handle in handles.values() { handle.on_disable(); }
        handles.clear();
    }

    /// #### 한국어
    /// 캐시된 에셋 핸들의 데이터가 아무런 이상이 없는 경우 `true`를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Return `true` if the data in the cached asset handle is ok.
    /// 
    #[inline]
    pub fn is_safety(&self) -> bool {
        self.safety.load(MemOrdering::Acquire)
    }
}
