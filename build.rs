mod assets {
    use std::include_str;
    use std::path::PathBuf;
    use std::collections::HashMap;

    use lazy_static::lazy_static;

    use super::utils;


    pub const KEY_DIR_REL_PATH_STR: &'static str = "./keys";
    pub const ASSET_DIR_REL_PATH_STR: &'static str = "./assets";
    lazy_static! {
        pub static ref ASSET_LISTS: HashMap<PathBuf, Types> = {
            let txt = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/AssetLists.txt"));
            parsing_asset_lists_txt(txt).unwrap()
        };
    }


    /// #### 한국어 </br>
    /// 에셋은 다음과 같은 에셋의 유형이 존재합니다. </br>
    /// - 정적: 에셋 파일의 내용이 바뀌지 않음을 의미합니다. </br>
    /// - 동적: 에셋 파일의 내용이 바뀔 수 있음을 의미합니다. </br>
    /// - 선택적: 에셋 파일이 존재하지 않을 수 있고, 에셋 파일이 생성될 수 있음을 의미합니다. </br>
    /// 
    /// #### English (Translation)
    /// There are the following asset types: </br>
    /// - Static: This means that the contents of the asset file do not change. </br>
    /// - Dynamic: This means that the contents of the asset file can change. </br>
    /// - Optional: This means the asset file may not exist, or an asset file may be created. </br>
    /// 
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Types {
        Static,
        Dynamic,
        Optional,
    }

    impl Types {
        /// #### 한국어 </br>
        /// 에셋이 읽기 가능한지 여부를 반환합니다. </br>
        /// 
        /// #### English (Translation)
        /// Returns whether the asset is readable </br>
        /// 
        #[allow(unused)]
        pub fn readable(&self) -> bool {
            match self {
                Self::Static => true,
                Self::Dynamic => true,
                Self::Optional => true,
            }
        }

        /// #### 한국어 </br>
        /// 에셋이 쓰기 가능한지 여부를 반환합니다. </br>
        /// 
        /// #### English (Translation)
        /// Returns whether the asset is writable </br>
        /// 
        pub fn writable(&self) -> bool {
            match self {
                Self::Static => false,
                Self::Dynamic => true,
                Self::Optional => true,
            }
        }

        /// #### 한국어 </br>
        /// 에셋이 생성 가능한지 여부를 반환합니다. </br>
        /// 
        /// #### English (Translation)
        /// Returns whether the asset is creatable </br>
        /// 
        pub fn creatable(&self) -> bool {
            match self {
                Self::Static => false,
                Self::Dynamic => false,
                Self::Optional => true,
            }
        }
    }


    /// #### 한국어 </br>
    /// `AssetLists.txt`파일을 구문분석하여 에셋 목록를 반환합니다. </br>
    /// 구문 분석 도중 오류가 발생할 경우 오류 메시지를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Parses the file `AssetLists.txt` and returns a list of assets. </br>
    /// If an error occurs during parsing, an error message is returned. </br>
    /// 
    pub fn parsing_asset_lists_txt(txt: &str) -> Result<HashMap<PathBuf, Types>, String> {
        const COMMENT_CH: char = '#';
        let mut list = HashMap::new();

        // (한국어) 텍스트 파일의 내용을 한 줄씩 읽습니다.
        // (English Translation) Read the contents of the text file line by line.
        'txt: for (line, line_str) in txt.lines().enumerate() {
            // (한국어) 한 줄의 내용에서 단어 단위로 읽습니다.
            // (English Translation) Read one line of content word by word.
            let mut path_str = String::new();
            let mut type_str = String::new();
            'line: for (idx, word) in line_str.trim().split_whitespace().enumerate() {
                for ch in word.chars() {
                    if ch == COMMENT_CH {
                        // (한국어) 주석 기호('#') 이후 문자는 읽지 않습니다.
                        // (English Translation) Characters after the comment sign ('#') are not read.
                        break 'line;
                    }

                    match idx {
                        0 => path_str.push(ch),
                        1 => type_str.push(ch),
                        _ => return Err(format!("invalid syntax. (line:{})", line + 1)),
                    }
                }
            }

            // (한국어) [1] <경로>와 <유형>이 모두 비어있는 경우. (예: 주석, 공백 라인)
            // (English Translation) [1] If <Path> and <Type> are empty. (e.g. comment, blank line)
            if path_str.is_empty() && type_str.is_empty() {
                // (한국어) 다음 라인으로 넘어갑니다.
                // (English Translation) Skip to the next line.
                continue 'txt;
            }
            // (한국어) [2] <경로>와 <유형>이 모두 존재하는 경우.
            // (English Translation) [2] If both <Path> and <Type> exist.
            else if !path_str.is_empty() && !type_str.is_empty() {
                let path = PathBuf::from(&path_str);

                // (한국어) <유형>이 유효한지 확인합니다.
                // (English Translation) Checks if <Type> is valid.
                let types = match type_str.as_str() {
                    "Static" => Types::Static,
                    "Dynamic" => Types::Dynamic,
                    "Optional" => Types::Optional,
                    _ => return Err(format!("invalid types. (line:{})", line + 1)),
                };

                // (한국어) <경로>가 루트 에셋 디렉토리의 하위 경로인지 확인합니다.
                // (English Translation) Checks if <Path> is a subpath of the root asset directory.
                if !utils::is_subpath(ASSET_DIR_REL_PATH_STR, &path) {
                    return Err(format!("This file is not in a subpath of the asset directory. (line:{})", line + 1));
                }

                // (한국어) <유형>이 선택적 유형이 아닌 경우 에셋 파일이 존재하는지 확인합니다.
                // (English Translation) If <Type> is not an optional type, check if the asset file exists.
                if !types.creatable() && !utils::file_exsist(ASSET_DIR_REL_PATH_STR, &path) {
                    return Err(format!("The given asset's path is not a file or cannot be found. (line:{})", line + 1));
                }
                
                // (한국어) 이미 리스트에 <경로>가 포함되어 있는지 확인합니다.
                // (English Translation) Check if the list already contains the <Path>.
                match list.contains_key(&path) {
                    false => list.insert(path, types),
                    true => return Err(format!("duplicate path. (line:{})", line + 1)),
                };
            }
            // (한국어) [3] <경로>와 <유형>중 하나만 존재하는 경우
            // (English Translation) [3] If only one of <Path> and <Type> exists.
            else {
                return Err(format!("invalid syntax. (line:{})", line + 1));
            }
        }

        Ok(list)
    }
}


mod utils {
    use std::path::{Path, PathBuf};

    pub fn is_subpath<R: AsRef<Path>, P: AsRef<Path>>(root: R, subpath: P) -> bool {
        use path_clean::clean;
    
        let root = clean(root);
        let path = clean(PathBuf::from_iter([root.as_ref(), subpath.as_ref()]));
        path.starts_with(root)
    }

    pub fn file_exsist<R: AsRef<Path>, P: AsRef<Path>>(root: R, subpath: P) -> bool {
        let path = PathBuf::from_iter([root.as_ref(), subpath.as_ref()])
            .canonicalize()
            .unwrap();
        path.is_file()
    }
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


/// #### 한국어 </br>
/// `AssetLists.txt`목록에 있는 에셋의 SHA256 해시 값을 생성하여 파일에 저장합니다. </br>
/// 정적 에셋 유형만 해시 값이 생성되며, 생성된 해시 값은 파일 무결성 검사에서 사용됩니다. </br>
/// 
/// #### English (Translation) </br>
/// Generates the SHA256 hash values of the assets in the 'AssetLists.txt' list and saves them to a file. </br>
/// Only static asset types have hash values generated, 
/// and the generated hash values are used in file integrity checks. </br>
/// 
fn gen_asset_sha256_keys() {
    use std::fs;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    use fs_extra::dir;
    use sha2::{Digest, Sha256};

    use assets::{
        KEY_DIR_REL_PATH_STR,
        ASSET_DIR_REL_PATH_STR,
        ASSET_LISTS,
    };


    // (한국어) 기존의 키값을 저장하는 디렉토리를 삭제하고 새로 생성합니다.
    // (English Translation) Delete the directory that stores the existing key value and create a new one.
    dir::create(KEY_DIR_REL_PATH_STR, true).expect("Failed to create asset key directory.");

    for (path, types) in ASSET_LISTS.iter() {
        // (한국어) `AssetLists.txt`목록에 있는 에셋 파일이 존재하는지 확인합니다.
        // (English) Checking if the asset file in the `AssetLists.txt` list exists.
        let asset_file_path = PathBuf::from_iter([Path::new(ASSET_DIR_REL_PATH_STR), path]);
        if !types.creatable() {
            assert!(asset_file_path.is_file(), "The given asset's path is not a file or cannot be found.");
        }

        // (한국어) 키 파일 경로의 디렉토리가 없는 경우 생성합니다.
        // (English) Creating the directory in the key file path if it does not exist.
        let key_file_path = PathBuf::from_iter([Path::new(KEY_DIR_REL_PATH_STR), path]);
        if let Some(key_dir_path) = key_file_path.parent() {
            if !key_dir_path.try_exists().is_ok_and(|flag| flag) {
                dir::create_all(key_dir_path, false).expect("Failed to create asset key subdirectory.")
            }
        }

        // (한국어) 정적 데이터 유형일경우 SHA256 해시 값을 생성하고 파일에 저장합니다.
        // (English) If it's a static datatype, generate a SHA256 hash value and save it to a file.
        if !types.writable() {
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


/// #### 한국어 </br>
/// 원본 에셋 파일을 빌드 대상 디렉토리에 복사합니다. </br>
/// `AssetList.txt`목록에 존재하는 에셋만 복사합니다. </br>
/// 
/// #### English </br>
/// Copy the original asset files to the build target directory. </br>
/// Copy only the assets that exist in the `AssetList.txt` list. </br>
/// 
fn copy_asset_to_target() {
    use std::env;
    use std::path::{Path, PathBuf};

    use fs_extra::{dir, file};

    use assets::{
        ASSET_DIR_REL_PATH_STR,
        ASSET_LISTS,
    };

    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let target_path = PathBuf::from(format!("target/{}", profile));
    if target_path.try_exists().is_ok_and(|flag| flag) {
        let target_asset_path = PathBuf::from_iter([&target_path, Path::new(ASSET_DIR_REL_PATH_STR)]);
        dir::create(&target_asset_path, true).expect("Failed to install asset directory.");

        for (path, types) in ASSET_LISTS.iter() {
            if types.creatable() {
                continue;
            }

            let asset_file_path = PathBuf::from_iter([Path::new(ASSET_DIR_REL_PATH_STR), path])
                .canonicalize().unwrap();
            assert!(asset_file_path.is_file(), "The given asset's path is not a file or cannot be found.");

            let target_asset_file_path = PathBuf::from_iter([&target_asset_path, path]);
            if let Some(target_asset_dir_path) = target_asset_file_path.parent() {
                if !target_asset_dir_path.try_exists().is_ok_and(|flag| flag) {
                    dir::create_all(target_asset_dir_path, false).expect("Failed to create target asset subdirectory.");
                }
            }

            file::copy(asset_file_path, &target_asset_file_path, &file::CopyOptions::default())
                .expect("Failed to copy assets.");
        }
        return;
    }

    let target_path = PathBuf::from(format!("target/{}/{}", target, profile));
    if target_path.try_exists().is_ok_and(|flag| flag) {
        let target_asset_path = PathBuf::from_iter([&target_path, Path::new(ASSET_DIR_REL_PATH_STR)]);
        dir::create(&target_asset_path, true).expect("Failed to install asset directory.");

        for (path, data_type) in ASSET_LISTS.iter() {
            if data_type.creatable() {
                continue;
            }

            let asset_file_path = PathBuf::from_iter([Path::new(ASSET_DIR_REL_PATH_STR), path])
                .canonicalize().unwrap();
            assert!(asset_file_path.is_file(), "The given asset's path is not a file or cannot be found.");

            let target_asset_file_path = PathBuf::from_iter([&target_asset_path, path]);
            if let Some(target_asset_dir_path) = target_asset_file_path.parent() {
                if !target_asset_dir_path.try_exists().is_ok_and(|flag| flag) {
                    dir::create_all(target_asset_dir_path, false).expect("Failed to create target asset subdirectory.");
                }
            }

            file::copy(asset_file_path, &target_asset_file_path, &file::CopyOptions::default())
                .expect("Failed to copy assets.");
        }
        return;
    }

    panic!("Target build directory is not found!");
}



fn main() {
    println!("cargo:rerun-if-changed=./assets.txt");
    println!("cargo:rerun-if-changed=AssetLists.txt");
    gen_asset_sha256_keys();
    copy_asset_to_target();
}
