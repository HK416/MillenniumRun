use std::mem;
use std::path::Path;
use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::sync::{Arc, Weak, RwLock};

use crate::{
    game_err,
    assets::{
        interface::{
            HandleInner,
            AssetDecoder,
            AssetEncoder,
        },
        types::Types,
    },
    system::error::{
        AppResult,
        GameError,
    },
};



/// #### 한국어 </br>
/// 정적 유형인 에셋 파일의 핸들 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Handle to an asset file of static type. </br>
/// 
#[derive(Debug)]
pub struct StaticHandle {
    bytes: Vec<u8>,
}

impl StaticHandle {
    /// #### 한국어 </br>
    /// 새로운 에셋 핸들의 내부 데이터를 생성합니다. </br>
    /// 이 함수에서 파일을 열고 내부 바이트 배열을 읽어옵니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates internal data for a new asset handle. </br>
    /// In this function, Developer open a file and read its internal byte array. </br>
    /// If an error occurs while executing the function, it returns `GameError`. </br>
    /// 
    pub(super) fn new<P: AsRef<Path>>(abs_path: P) -> AppResult<Self> {
        log::info!("load static asset :: <Path:{}>", abs_path.as_ref().display());
        let mut file = OpenOptions::new()
            .read(true)
            .open(abs_path)
            .map_err(|e| game_err!(
                "Asset load failed",
                "Opening the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|e| game_err!(
                "Asset load failed",
                "Reading the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        if bytes.is_empty() {
            return Err(game_err!(
                "Asset load failed",
                "The content of the asset file is empty!"
            ));
        }

        Ok(Self { bytes })
    }
}

impl HandleInner for StaticHandle {
    #[inline]
    fn read<T, D>(&self, decoder: &D) -> AppResult<D::Output> 
        where D: AssetDecoder<Output = T> {
        decoder.decode(&self.bytes)
    }

    #[inline]
    fn read_or_default<T, D, E>(&mut self, encoder: &E, decoder: &D) -> AppResult<D::Output>
        where T: Default, D: AssetDecoder<Output = T>, E: AssetEncoder<Input = T> {
        decoder.decode(&self.bytes)
    }

    #[inline]
    fn write<T, E>(&mut self, encoder: &E, value: &E::Input) -> AppResult<()>
        where E: AssetEncoder<Input = T> {
        log::warn!("Asset files of static type do not perform write functions!");
        Ok(())
    }
}



/// #### 한국어 </br>
/// 동적 유형인 에셋 파일의 핸들 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Handle to an asset file of dynamic type. </br>
/// 
#[derive(Debug)]
pub struct DynamicHandle {
    file: File,
    bytes: Vec<u8>,
}

impl DynamicHandle {
    /// #### 한국어 </br>
    /// 새로운 에셋 핸들의 내부 데이터를 생성합니다. </br>
    /// 이 함수에서 파일을 열고 내부 바이트 배열을 읽어옵니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates internal data for a new asset handle. </br>
    /// In this function, Developer open a file and read its internal byte array. </br>
    /// If an error occurs while executing the function, it returns `GameError`. </br>
    /// 
    pub(super) fn new<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        log::info!("load dynamic asset :: <Path:{}>", path.as_ref().display());
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|e| game_err!(
                "Asset load failed",
                "Opening the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|e| game_err!(
                "Asset load failed",
                "Reading the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        if bytes.is_empty() {
            return Err(game_err!(
                "Asset load failed",
                "The content of the asset file is empty!"
            ));
        }

        Ok(Self { file, bytes })
    }
}

impl HandleInner for DynamicHandle {
    #[inline]
    fn read<T, D>(&self, decoder: &D) -> AppResult<D::Output> 
        where D: AssetDecoder<Output = T> {
        decoder.decode(&self.bytes)
    }

    #[inline]
    fn read_or_default<T, D, E>(&mut self, encoder: &E, decoder: &D) -> AppResult<D::Output>
        where T: Default, D: AssetDecoder<Output = T>, E: AssetEncoder<Input = T> {
        decoder.decode(&self.bytes)
    }

    #[inline]
    fn write<T, E>(&mut self, encoder: &E, value: &E::Input) -> AppResult<()>
        where E: AssetEncoder<Input = T> {
        self.bytes = encoder.encode(value)?;
        self.file.set_len(self.bytes.len() as u64)
            .map_err(|e| game_err!(
                "Writing the asset file failed",
                "Writing the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        self.file.seek(SeekFrom::Start(0))
            .map_err(|e| game_err!(
                "Writing the asset file failed",
                "Writing the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        self.file.write_all(&self.bytes)
            .map_err(|e| game_err!(
                "Writing the asset file failed",
                "Writing the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        Ok(())
    }
}


/// #### 한국어 </br>
/// 선택적 유형인 에셋 파일의 핸들 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Handle to an asset file of optional type. </br>
/// 
#[derive(Debug)]
pub struct OptionalHandle {
    file: File,
    bytes: Vec<u8>,
}

impl OptionalHandle {
    /// #### 한국어 </br>
    /// 새로운 에셋 핸들의 내부 데이터를 생성합니다. </br>
    /// 이 함수에서 파일을 열고 내부 바이트 배열을 읽어옵니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates internal data for a new asset handle. </br>
    /// In this function, Developer open a file and read its internal byte array. </br>
    /// If an error occurs while executing the function, it returns `GameError`. </br>
    /// 
    pub(super) fn new<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        log::info!("load dynamic asset :: <Path:{}>", path.as_ref().display());
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())
            .map_err(|e| game_err!(
                "Asset load failed",
                "Opening the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|e| game_err!(
                "Asset load failed",
                "Reading the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;

        Ok(Self { file, bytes })
    }
}

impl HandleInner for OptionalHandle {
    #[inline]
    fn read<T, D>(&self, decoder: &D) -> AppResult<D::Output> 
        where D: AssetDecoder<Output = T> {
        decoder.decode(&self.bytes)
    }

    #[inline]
    fn read_or_default<T, D, E>(&mut self, encoder: &E, decoder: &D) -> AppResult<D::Output>
        where T: Default, D: AssetDecoder<Output = T>, E: AssetEncoder<Input = T> {
        if self.bytes.is_empty() {
            let value = T::default();
            self.write(encoder, &value)?;
            Ok(value)
        } else {
            decoder.decode(&self.bytes)
        }
    }

    #[inline]
    fn write<T, E>(&mut self, encoder: &E, value: &E::Input) -> AppResult<()>
        where E: AssetEncoder<Input = T> {
        self.bytes = encoder.encode(value)?;
        self.file.set_len(self.bytes.len() as u64)
            .map_err(|e| game_err!(
                "Writing the asset file failed",
                "Writing the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        self.file.seek(SeekFrom::Start(0))
            .map_err(|e| game_err!(
                "Writing the asset file failed",
                "Writing the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        self.file.write_all(&self.bytes)
            .map_err(|e| game_err!(
                "Writing the asset file failed",
                "Writing the asset file failed for the following reasons: {}",
                e.to_string()
            ))?;
        Ok(())
    }
}



/// #### 한국어 </br>
/// 에셋 파일을 읽거나 쓸 수 있는 핸들입니다. </br>
/// 여러 스레드에서 공유해서 사용할 수 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// A handle to read or write to the asset file. </br>
/// It can be shared and used across multiple threads. </br>
/// 
#[derive(Debug, Clone)]
pub enum AssetHandle {
    Static(Arc<RwLock<StaticHandle>>),
    Dynamic(Arc<RwLock<DynamicHandle>>),
    Optional(Arc<RwLock<OptionalHandle>>),
}

impl AssetHandle {
    const ERR_ACCESS_FAILED: &'static str = "Failed to access asset internal data.";

    #[inline]
    pub fn strong_count(&self) -> usize {
        match self {
            AssetHandle::Static(this) => Arc::strong_count(this),
            AssetHandle::Dynamic(this) => Arc::strong_count(this),
            AssetHandle::Optional(this) => Arc::strong_count(this),
        }
    }

    #[inline]
    pub fn types(&self) -> Types {
        match self {
            AssetHandle::Static(_) => Types::Static,
            AssetHandle::Dynamic(_) => Types::Dynamic,
            AssetHandle::Optional(_) => Types::Optional,
        }
    }

    /// #### 한국어 </br>
    /// 에셋의 바이트 배열을 주어진 디코더로 디코딩하여 결과를 반환합니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Decode a byte array of assets with the given decoder and returns the result. </br>
    /// If an error occurs while executing the function, it returns `GameError`. </br>
    /// 
    pub fn read<T, D>(&self, decoder: &D) -> AppResult<D::Output>
    where D: AssetDecoder<Output = T> {
        match self {
            AssetHandle::Static(handle) => {
                handle.read()
                    .expect(Self::ERR_ACCESS_FAILED)
                    .read(decoder)
            },
            AssetHandle::Dynamic(handle) => {
                handle.read()
                    .expect(Self::ERR_ACCESS_FAILED)
                    .read(decoder)
            },
            AssetHandle::Optional(handle) => {
                handle.read()
                .expect(Self::ERR_ACCESS_FAILED)
                .read(decoder)
            }
        }
    }

    /// #### 한국어 </br>
    /// 에셋의 바이트 배열을 주어진 디코더로 디코딩하여 결과를 반환합니다. </br>
    /// 선택적 유형인 경우 에셋 파일의 바이트 배열이 비어있을 때 
    /// 주어진 기본 값을 에셋 파일에 쓴 뒤 반환합니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Decode a byte array of assets with the given decoder and returns the result. </br>
    /// If this is an optional type, if the byte array in the asset file is empty,
    /// the given default value will be written to the asset and returned. </br>
    /// If an error occurs while executing the function, it returns `GameError`. </br>
    /// 
    pub fn read_or_default<T, D, E>(&self, encoder: &E, decoder: &D) -> AppResult<D::Output> 
    where T: Default, D: AssetDecoder<Output = T>, E: AssetEncoder<Input = T> {
        match self {
            AssetHandle::Static(handle) => {
                handle.write()
                    .expect(Self::ERR_ACCESS_FAILED)
                    .read_or_default(encoder, decoder)
            },
            AssetHandle::Dynamic(handle) => {
                handle.write()
                    .expect(Self::ERR_ACCESS_FAILED)
                    .read_or_default(encoder, decoder)
            },
            AssetHandle::Optional(handle) => {
                handle.write()
                .expect(Self::ERR_ACCESS_FAILED)
                .read_or_default(encoder, decoder)
            }
        }
    }

    /// #### 한국어 </br>
    /// 에셋의 바이트 배열을 주어진 데이터로 채우고 에셋 파일에 덮어 씁니다. </br>
    /// 정적 유형의 에셋 파일의 경우 아무것도 수행하지 않습니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `GameError`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Fill the asset's byte array with the given data and overwrites its in the asset file. </br>
    /// For asset files of static type, it does nothing. </br>
    /// If an error occurs while executing the function, it returns `GameError`. </br>
    /// 
    pub fn write<T, E>(&self, encoder: &E, value: &E::Input) -> AppResult<()> 
    where E: AssetEncoder<Input = T> {
        match self {
            AssetHandle::Static(handle) => {
                handle.write()
                    .expect(Self::ERR_ACCESS_FAILED)
                    .write(encoder, value)
            },
            AssetHandle::Dynamic(handle) => {
                handle.write()
                    .expect(Self::ERR_ACCESS_FAILED)
                    .write(encoder, value)
            },
            AssetHandle::Optional(handle) => {
                handle.write()
                .expect(Self::ERR_ACCESS_FAILED)
                .write(encoder, value)
            }
        }
    }

    /// #### 한국어 </br>
    /// 내부 참조 횟수를 증가시키지 않고 에셋 파일 핸들을 복제합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Duplicate the asset file handle without incrementing the internal reference count. </br>
    /// 
    #[inline]
    pub fn downgrade(&self) -> WeakAssetHandle {
        match self {
            AssetHandle::Static(this) => WeakAssetHandle::Static(Arc::downgrade(this)),
            AssetHandle::Dynamic(this) => WeakAssetHandle::Dynamic(Arc::downgrade(this)),
            AssetHandle::Optional(this) => WeakAssetHandle::Optional(Arc::downgrade(this)),
        }
    }
}



/// #### 한국어 </br>
/// 에셋 파일의 핸들입니다. 핸들을 복제하여 여러 스레드에서 공유하여 사용할 수 있습니다. </br>
/// 에셋 파일 핸들의 기능을 사용하려면 `upgrade`함수를 사용해 `AssetHandle`로 업그레이드 해야 합니다. </br>
/// 
/// #### English (Translation) </br>
/// A handle to the asset file. By duplicating the handle, it can be shared and used by multiple threads. </br>
/// To use the functionality of asset file handles, you need to upgrade them to `AssetHandle` using `upgrade` function. </br>
/// 
#[derive(Debug, Clone)]
pub enum WeakAssetHandle{
    Static(Weak<RwLock<StaticHandle>>),
    Dynamic(Weak<RwLock<DynamicHandle>>),
    Optional(Weak<RwLock<OptionalHandle>>),
}

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
        Some(match self {
            WeakAssetHandle::Static(this) => AssetHandle::Static(this.upgrade()?),
            WeakAssetHandle::Dynamic(this) => AssetHandle::Dynamic(this.upgrade()?),
            WeakAssetHandle::Optional(this) => AssetHandle::Optional(this.upgrade()?),
        })
    }
}
