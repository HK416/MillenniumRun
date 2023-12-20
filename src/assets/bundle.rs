use std::io;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{self, Receiver};
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};

use sha2::{Digest, Sha256};
use notify::{
    Event,
    EventKind,
    event::ModifyKind,
    Config,
    Watcher,
    RecursiveMode,
    RecommendedWatcher,
    Result as NotifyResult,
};

use crate::{
    game_err,
    assets::{
        handle::{
            AssetHandle,
            StaticHandle,
            DynamicHandle,
            OptionalHandle,
        },
        list::{
            ASSET_LISTS,
            AssetKeys,
        },
        path::ROOT_ASSET_PATH,
        types::Types,
    },
    system::error::{
        AppResult,
        GameError,
    },
};


const ERR_TITLE_VERIFICATION_FAILED: &'static str = "Asset verification failed";
const ERR_TITLE_WATCHER_INIT_FAILED: &'static str = "Asset file watcher initialize failed";
const ERR_VERIFICATION_FAILED: &'static str = "Asset file verification failed for the following reasons:";
const ERR_WATCHER_INIT_FAILED: &'static str = "Asset file watcher initialization failed for following reasons:";



/// #### 한국어 </br>
/// 어플리케이션의 에셋 파일을 관리합니다. </br>
/// 에셋파일을 읽거나 쓰고, 에셋파일을 감시합니다. </br>
/// 
/// #### English (Translation) </br>
/// Manage the asset files of the application. </br>
/// Reads or writes asset files and monitors asset files. </br>
/// 
#[derive(Debug, Clone)]
pub struct AssetBundle {
    root_path: PathBuf,
    asset_list: Arc<HashMap<PathBuf, Types>>,
    loaded_assets: Arc<RwLock<HashMap<PathBuf, AssetHandle>>>,
    integrity_flag: Arc<AtomicBool>,
}

impl AssetBundle {
    pub fn new() -> AppResult<Self> {
        let root_path = ROOT_ASSET_PATH.clone()?;
        let asset_list = Arc::new(ASSET_LISTS.clone()?);
        let loaded_assets = Arc::new(RwLock::new(HashMap::with_capacity(asset_list.len())));
        
        // (한국어) 에셋 파일 감시자를 생성합니다.
        // (English Translation) Create an asset file watcher.
        let (sender, receiver) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(sender, Config::default())
            .map_err(|e| game_err!(
                ERR_TITLE_WATCHER_INIT_FAILED, "{} {}", ERR_WATCHER_INIT_FAILED, e.to_string()
            ))?;
        watcher.watch(&root_path, RecursiveMode::Recursive)
            .map_err(|e| game_err!(
                ERR_TITLE_WATCHER_INIT_FAILED, "{} {}", ERR_WATCHER_INIT_FAILED, e.to_string()
            ))?;

        // (한국어) 에셋 파일 감시를 시작합니다.
        // (English Translation) Start monitoring asset files.
        let integrity_flag = Arc::new(AtomicBool::new(true));
        let integrity_flag_cloned = integrity_flag.clone();
        let root_path_cloned = root_path.clone();
        let asset_list_cloned = asset_list.clone();
        thread::spawn(move || {
            watcher_main(watcher, receiver, root_path_cloned, asset_list_cloned);
            integrity_flag_cloned.store(false, MemOrdering::Release);
        });

        // (한국어) 에셋 파일 검사를 시작합니다.
        // (English Translation) Start checking asset files.
        check_assets(&root_path, &asset_list)?;


        Ok(Self { root_path, asset_list, loaded_assets, integrity_flag })
    }

    /// #### 한국어 </br>
    /// 에셋 파일에 이상이 없는 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// If there is no problem with the asset file, it returns `true`. </br>
    /// 
    #[inline]
    pub fn check_integrity(&self) -> bool {
        self.integrity_flag.load(MemOrdering::Acquire)
    }

    /// #### 한국어 </br>
    /// 에셋 파일의 핸들을 가져옵니다. </br>
    /// 핸들을 가져오는 도중 오류가 발생한 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the handle to the asset file. </br>
    /// Returns `PanicMsg` if an error occurred while retrieving the handle. </br>
    /// 
    pub fn get<P: AsRef<Path>>(&self, rel_path: P) -> AppResult<AssetHandle> {
        {
            let loaded_assets = self.loaded_assets
                .read()
                .expect("Failed to access loaded assets.");
            if let Some(handle) = loaded_assets.get(rel_path.as_ref()) {
                return Ok(handle.clone())
            }
        }

        {
            if let Some(types) = self.asset_list.get(rel_path.as_ref()) {
                let abs_path = PathBuf::from_iter([&self.root_path, rel_path.as_ref()]);
                let handle = match types {
                    Types::Static => AssetHandle::Static(Arc::new(RwLock::new(StaticHandle::new(abs_path)?))),
                    Types::Dynamic => AssetHandle::Dynamic(Arc::new(RwLock::new(DynamicHandle::new(abs_path)?))),
                    Types::Optional => AssetHandle::Optional(Arc::new(RwLock::new(OptionalHandle::new(abs_path)?))),
                };

                let mut loaded_assets = self.loaded_assets
                    .write()
                    .expect("Failed to access loaded assets.");
                loaded_assets.insert(rel_path.as_ref().into(), handle.clone());
                return Ok(handle);
            }
        }

        Err(game_err!(
            "Failed to get asset handle",
            "The asset path given in the asset list does no exist."
        ))
    }

    
    /// #### 한국어 </br>
    /// 로드된 에셋 목록에서 주어진 경로의 에셋을 제거합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Removes the asset at the given path from the list of loaded assets. </br>
    /// 
    pub fn release<P: AsRef<Path>>(&self, rel_path: P) {
        self.loaded_assets
            .write()
            .expect("Failed to access loaded assets.")
            .remove(rel_path.as_ref());
    }
}



/// #### 한국어 </br>
/// 에셋 파일을 감시하는 루프입니다. </br>
/// 에셋 파일의 데이터가 손상된 경우, 혹은 오류가 발생한 경우 
/// 프로그램 실행 중에 이 루프를 빠져나옵니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a loop that monitors asset files. </br>
/// If the data in the asset file is corrupted, or an error occurs, 
/// this loop will be exited during program execution. </br>
/// 
fn watcher_main(
    _watcher: RecommendedWatcher, 
    receiver: Receiver<NotifyResult<Event>>,
    root_path: PathBuf,
    asset_list: Arc<HashMap<PathBuf, Types>>,
) {
    log::info!("Start monitoring asset files.");

    // (한국어) 에셋 파일 감시자로부터 받아온 이벤트를 처리합니다.
    // (English Translation) Processes events received from the asset file watcher. 
    for result in receiver {
        let event = match result {
            Ok(event) => event,
            Err(e) => {
                log::error!("Asset file watcher has stopped for the following reasones: {}", e.to_string());
                return;
            }
        };

        match &event.kind {
            EventKind::Modify(kind) => match kind {
                // (한국어) 파일 데이터가 수정됬다는 이벤트를 수신한 경우.
                // (English Trnaslation) If an event that file data has been modified is received.
                ModifyKind::Data(_) => {
                    // (한국어) 
                    // 해당 파일이 에셋 리스트에 포함되어 있는지 확인합니다.
                    // 포함되어 있는 경우 에셋 파일의 유형을 확인합니다.
                    // 
                    // (English Translation) 
                    // Checks if the file is included in the asset list.
                    // Checks the type of asset file, if included.
                    // 
                    for path in event.paths.iter() {
                        let path = match get_subpath(&path, &root_path) {
                            Ok(path) => path,
                            Err(e) => {
                                log::error!("Asset file watcher has stopped for the following reasones: {}", e.to_string());
                                return;
                            }
                        };

                        if let Some(types) = asset_list.get(path) {
                            if !types.writable() {
                                log::error!("[MODIFY] The data in the asset file is corrupted! (file:{})", path.display());
                                return;
                            }
                        }
                    }
                },
                _ => { }
            },
            _ => { }
        };
    }

    log::info!("Finish monitoring asset files.");
}



#[inline]
fn get_subpath<'a>(path: &'a Path, base: &'a Path) -> Result<&'a Path, String> {
    path.strip_prefix(base).map_err(|e| e.to_string())
}



/// #### 한국어 </br>
/// `AssetLists.txt`목록의 에셋을 확인합니다. </br>
/// `AssetLists.txt`목록의 에셋 중 정적 유형의 에셋 파일은 
/// 컴파일 타임에 바이너리에 키값이 저장됩니다. </br>
/// 모든 정적 및 동적 유형의 에셋 파일이 존재하는 검사하고, 
/// 정적 유형의 에셋파일에 대해 키값이 일치하는지 검사합니다. </br>
/// 검사 도중 오류가 발생한 경우 `PanicMsg`를 반환합니다. </br>
/// 
/// 
/// #### English (Translation) </br>
/// Check the assets in the `AssetLists.txt` list. </br>
/// Among the assets in the `AssetLists.txt` list, 
/// the key value of static type asset files are stored in the binary at compile time. </br>
/// Checks if all static and dynamic type asset files exist 
/// and checks if the key values match for static type asset files. </br>
/// If an error occurs during the check, it returns `PanicMsg`. </br>
/// 
fn check_assets(
    root_path: &Path, 
    asset_list: &HashMap<PathBuf, Types>,
) -> AppResult<()> {
    let mut handles = Vec::with_capacity(asset_list.len());

    for (rel_path, types) in asset_list.iter() {
        let types_cloned = types.clone();
        let rel_path_cloned = rel_path.clone();
        let root_path_cloned = root_path.to_path_buf().clone();
        handles.push(thread::spawn(move || {
            let abs_path = PathBuf::from_iter([&root_path_cloned, &rel_path_cloned]);
            if !types_cloned.creatable() && !abs_path.is_file() {
                return Err(game_err!(
                    ERR_TITLE_VERIFICATION_FAILED,
                    "{} {}",
                    ERR_VERIFICATION_FAILED,
                    "Asset is not a file or path cannot be found!"
                ));
            }
            
            if !types_cloned.writable() {
                let key_file = AssetKeys::get(
                    rel_path_cloned.to_str().unwrap()
                ).ok_or_else(|| game_err!(
                    ERR_TITLE_VERIFICATION_FAILED,
                    "{} {}",
                    ERR_VERIFICATION_FAILED,
                    "Asset key not found!"
                ))?;
                
                let hash = {
                    let mut file = OpenOptions::new()
                    .read(true)
                    .open(abs_path)
                    .map_err(|e| game_err!(
                        ERR_TITLE_VERIFICATION_FAILED,
                        "{} {}",
                        ERR_VERIFICATION_FAILED,
                        e.to_string()
                    ))?;
                    let mut hasher = Sha256::new();
                    io::copy(&mut file, &mut hasher)
                    .map_err(|e| game_err!(
                        ERR_TITLE_VERIFICATION_FAILED,
                        "{} {}",
                        ERR_VERIFICATION_FAILED,
                        e.to_string()
                    ))?;
                    hasher.finalize()
                };
                
                if key_file.data.as_ref().ne(hash.as_slice()) {
                    return Err(game_err!(
                        ERR_TITLE_VERIFICATION_FAILED,
                        "{} {}",
                        ERR_VERIFICATION_FAILED,
                        "Key values in asset files do not match!"
                    ));
                }
            }

            Ok(())
        }));
    }

    for th in handles { 
        th.join().unwrap()?;
    }

    Ok(())
}
