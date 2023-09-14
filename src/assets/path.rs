use std::env;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

use crate::{
    panic_msg,
    app::abort::{PanicMsg, AppResult},
};


const ERR_TITLE: &'static str = "Failed to get asset directory path";
const ERR_MESSAGE: &'static str = "Failed to get asset directory path for the following reasons:";
const ERR_NOT_FOUND: &'static str = "Path not found.";
const ERR_NOT_DIRECTORY: &'static str = "The path is not a directory.";

/// #### 한국어 </br>
/// 에셋 디렉토리의 상대 경로 입니다. </br>
/// 에셋 디렉토리는 실행 파일의 상대경로에 위치해야 합니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a relative path to the asset directory. </br>
/// The asset directory must be located in a relative path to the executable file. </br>
/// 
const ASSETS_REL_PATH_STR: &'static str = "./assets";

lazy_static! {
    pub(super) static ref ROOT_ASSET_PATH: AppResult<PathBuf> = {
        let result = {
            let asset_dir = PathBuf::from_iter([
                env::current_exe()
                    .map_err(|e| panic_msg!(ERR_TITLE, "{} {}", ERR_MESSAGE, e.to_string()))?
                    .parent()
                    .ok_or_else(|| panic_msg!(ERR_TITLE, "{} {}", ERR_MESSAGE, ERR_NOT_FOUND))?,
                Path::new(ASSETS_REL_PATH_STR),
            ])
            .canonicalize()
            .map_err(|e| panic_msg!(ERR_TITLE, "{} {}", ERR_MESSAGE, e.to_string()))?;

            match asset_dir.is_dir() {
                true => Ok(asset_dir),
                false => Err(panic_msg!(ERR_TITLE, "{} {}", ERR_MESSAGE, ERR_NOT_DIRECTORY))
            }
        };

        match &result {
            Ok(path) => log::info!("Asset directory path: {}", path.display()),
            Err(_) => log::warn!("Asset directory path not found!"),
        }

        return result;
    };
}
