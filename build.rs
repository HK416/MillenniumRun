use std::fs;
use std::env;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use fs_extra::{
    dir::{create, create_all}, 
    file::{copy, CopyOptions}
};
use path_clean::clean;
use lazy_static::lazy_static;

const KEY_PATH: &'static str = "keys";
const ASSET_PATH: &'static str = "assets";

lazy_static! {
    static ref ASSET_LIST: HashMap<PathBuf, AssetDataType> = {
        let buf = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/AssetLists.txt"));
        parsing_asset_lists(buf)
    };
}


/// #### 한국어
/// `AssetLists.txt`를 구문분석하는 함수입니다.
/// 구문 분석 도중 문법 오류가 발생할 경우 오류가 발생한 줄을 사용자에게 알리고, 프로그램 실행을 중단시킵니다.
/// 
/// #### English (Translation)
/// A function that parses `AssetLists.txt`
/// If a syntax error occurs during parsing, the user is notified of the erroneous line and program execution is abort.
/// 
fn parsing_asset_lists(buf: &str) -> HashMap<PathBuf, AssetDataType> {
    const COMMENT: char = '#';
    let mut list = HashMap::new();
    for (num, line) in buf.lines().enumerate() {
        let mut collect_str = [String::with_capacity(256), String::with_capacity(16)];
        'line: for (cnt, word) in line.trim().split_whitespace().enumerate() {
            for ch in  word.chars() {
                if ch == COMMENT { 
                    // (한국어) 주석 기호`#` 이후의 문자를 건너뜁니다.
                    // (English) Characters after the comment sign `#` are skipped.
                    break 'line; 
                }
                else if cnt < 2 {
                    collect_str[cnt].push(ch);
                }
                else {
                    // (한국어) <경로> <유형> 이후 다른 문자가 있을 경우 프로그램 실행을 중단시킵니다.
                    // (English) If there are any other characters after <PATH> <TYPE>, abort program execution.
                    panic!("AssetLists.txt error: invalid syntax. (line:{})", num + 1);
                }
            }
        }

        if collect_str.iter().all(|str| str.len() == 0) {
            // (한국어) <경로>와 <유형>이 비어있는 경우 (예: 주석, 공백 라인) 다음 라인으로 넘어갑니다.
            // (English) If <PATH> and <TYPE> are empty (e.g. comment, blank line), skip to the next line.
            continue;
        }
        else if collect_str.iter().all(|str| str.len() > 0) {
            // (한국어) 
            // <경로>와 <유형>을 모두 가진 경우 리스트에 넣습니다.
            // 올바르지 않은 <유형>이거나, 이미 <경로>가 존재하는 경우, <경로>가 `assets`디렉토리의 하위 경로가 아닐 경우 프로그램 실행을 중단시킵니다.
            // 
            // (English) 
            // If it has both <path> and <type>, put it in the list.
            // Abort program execution if <type> is invalid, if <path> already exists, or if <path> is not a subpath of the `assets` directory.
            //
            let data_type = match collect_str[1].as_str() {
                "Static" => AssetDataType::StaticData,
                "Dynamic" => AssetDataType::DynamicData,
                "Optional" => AssetDataType::OptionalData,
                _ => { panic!("AssetLists.txt error: invalid types. (line:{})", num + 1) },
            };
            
            assert!(check_subpath(ASSET_PATH, &collect_str[0]), 
            "The file is not in a subpath of the assets directory. (line:{})", num + 1);
            if !data_type.is_optional() {
                assert!(check_file_exsist(ASSET_PATH, &collect_str[0]), 
                "The given asset's path is not a file or cannot be found. (line:{})", num + 1);
            }

            if list.insert(PathBuf::from(collect_str[0].as_str()), data_type,).is_some() {
                panic!("AssetLists.txt error: duplicate path. (line:{})", num + 1);
            }
        }
        else {
            // (한국어) <경로>와 <유형>중 하나만 존재하는 경우 프로그램 실행을 중단시킵니다.
            // (English) Abort program execution if only one of <path> and <type> exists.
            panic!("AssetLists.txt error: invalid syntax. (line:{})", num + 1);
        }
    }

    return list;
}

#[inline]
fn check_subpath<R: AsRef<Path>, P: AsRef<Path>>(root: R, subpath: P) -> bool {
    let root = clean(root);
    let path = clean(PathBuf::from_iter([root.as_ref(), subpath.as_ref()]));
    path.starts_with(root)
}

fn check_file_exsist<R: AsRef<Path>, P: AsRef<Path>>(root: R, subpath: P) -> bool {
    let path = PathBuf::from_iter([root.as_ref(), subpath.as_ref()])
        .canonicalize()
        .unwrap();
    path.is_file()
}


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


/// #### 한국어
/// `AssetLists.txt`목록에 있는 에셋의 SHA256 해시 값을 생성하여 파일에 저장합니다.
/// 생성된 SHA256 해시 값은 파일 무결성 검사에서 사용됩니다.
/// 해시 값은 정적 데이터 유형인 경우에만 생성됩니다.
/// 
/// #### English 
/// Generates the SHA256 hash values of the assets in the 'AssetLists.txt' list and saves them to a file.
/// The generated SHA256 hash value is used in file integrity checks.
/// Hash values are generated only if they are static data types.
/// 
fn gen_asset_sha256_keys() {
    // (한국어) 기존의 키값을 저장하는 디렉토리를 삭제하고 새로 생성합니다.
    // (English) Delete the directory that stores the existing key value and create a new one.
    create(KEY_PATH, true).expect("Failed to create asset key directory.");

    for (path, data_type) in ASSET_LIST.iter() {
        // (한국어) `AssetLists.txt`목록에 있는 에셋 파일이 존재하는지 확인합니다.
        // (English) Checking if the asset file in the `AssetLists.txt` list exists.
        let asset_file_path = PathBuf::from_iter([Path::new(ASSET_PATH), path]);
        if !data_type.is_optional() {
            assert!(asset_file_path.is_file(), "The given asset's path is not a file or cannot be found.");
        }

        // (한국어) 키 파일 경로의 디렉토리가 없는 경우 생성합니다.
        // (English) Creating the directory in the key file path if it does not exist.
        let key_file_path = PathBuf::from_iter([Path::new(KEY_PATH), path]);
        if let Some(key_dir_path) = key_file_path.parent() {
            if !key_dir_path.try_exists().is_ok_and(|flag| flag) {
                create_all(key_dir_path, false).expect("Failed to create asset key subdirectory.")
            }
        }

        // (한국어) 정적 데이터 유형일경우 SHA256 해시 값을 생성하고 파일에 저장합니다.
        // (English) If it's a static datatype, generate a SHA256 hash value and save it to a file.
        if data_type.is_static_data() {
            let hash = {
                let mut asset_file = fs::File::open(&asset_file_path)
                    .expect("Failed to open asset file.");
                let mut hasher = Sha256::new();
                io::copy(&mut asset_file, &mut hasher).unwrap();
                hasher.finalize()
            };

            let mut key_file = fs::File::create(key_file_path)
                .expect("Failed to create key file.");
            key_file.write_all(hash.as_slice()).expect("Failed to write file.");
        }
    }
}


/// #### 한국어
/// 원본 에셋 파일을 빌드 대상 디렉토리에 복사합니다.
/// `AssetList.txt`목록에 존재하는 에셋만 복사합니다.
/// 
/// #### English
/// Copy the original asset files to the build target directory.
/// Copy only the assets that exist in the `AssetList.txt` list.
/// 
fn copy_asset_to_target() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let target_path = PathBuf::from(format!("target/{}", profile));
    if target_path.try_exists().is_ok_and(|flag| flag) {
        let target_asset_path = PathBuf::from_iter([&target_path, Path::new(ASSET_PATH)]);
        create(&target_asset_path, true).expect("Failed to install asset directory.");

        for (path, data_type) in ASSET_LIST.iter() {
            if data_type.is_optional() {
                continue;
            }

            let asset_file_path = PathBuf::from_iter([Path::new(ASSET_PATH), path])
                .canonicalize().unwrap();
            assert!(asset_file_path.is_file(), "The given asset's path is not a file or cannot be found.");

            let target_asset_file_path = PathBuf::from_iter([&target_asset_path, path]);
            if let Some(target_asset_dir_path) = target_asset_file_path.parent() {
                if !target_asset_dir_path.try_exists().is_ok_and(|flag| flag) {
                    create_all(target_asset_dir_path, false).expect("Failed to create target asset subdirectory.");
                }
            }

            copy(asset_file_path, &target_asset_file_path, &CopyOptions::default())
                .expect("Failed to copy assets.");
        }
        return;
    }

    let target_path = PathBuf::from(format!("target/{}/{}", target, profile));
    if target_path.try_exists().is_ok_and(|flag| flag) {
        let target_asset_path = PathBuf::from_iter([&target_path, Path::new(ASSET_PATH)]);
        create(&target_asset_path, true).expect("Failed to install asset directory.");

        for (path, data_type) in ASSET_LIST.iter() {
            if data_type.is_optional() {
                continue;
            }

            let asset_file_path = PathBuf::from_iter([Path::new(ASSET_PATH), path])
                .canonicalize().unwrap();
            assert!(asset_file_path.is_file(), "The given asset's path is not a file or cannot be found.");

            let target_asset_file_path = PathBuf::from_iter([&target_asset_path, path]);
            if let Some(target_asset_dir_path) = target_asset_file_path.parent() {
                if !target_asset_dir_path.try_exists().is_ok_and(|flag| flag) {
                    create_all(target_asset_dir_path, false).expect("Failed to create target asset subdirectory.");
                }
            }

            copy(asset_file_path, &target_asset_file_path, &CopyOptions::default())
                .expect("Failed to copy assets.");
        }
        return;
    }

    panic!("Target build directory is not found!");
}

fn main() {
    println!("cargo:rerun-if-changed=AssetLists.txt");
    gen_asset_sha256_keys();
    copy_asset_to_target();
}
