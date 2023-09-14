use std::include_str;
use std::path::PathBuf;
use std::collections::HashMap;

use rust_embed::RustEmbed;
use lazy_static::lazy_static;

use crate::{
    panic_msg,
    app::abort::{PanicMsg, AppResult},
    assets::types::Types,
};



#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/keys/"]
pub(super) struct AssetKeys;

lazy_static! {
    pub(super) static ref ASSET_LISTS: AppResult<HashMap<PathBuf, Types>> = {
        let txt = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/AssetLists.txt"));
        parsing_asset_lists_txt(txt).map_err(|e| panic_msg!(
            "Failed to initialize asset list",
            "Asset list initialization failed for the following reasons: {}", e.as_str()
        ))
    };
}



/// #### 한국어 </br>
/// `AssetLists.txt`파일을 구문분석하여 에셋 목록를 반환합니다. </br>
/// 구문 분석 도중 오류가 발생할 경우 오류 메시지를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Parses the file `AssetLists.txt` and returns a list of assets. </br>
/// If an error occurs during parsing, an error message is returned. </br>
/// 
fn parsing_asset_lists_txt(txt: &str) -> Result<HashMap<PathBuf, Types>, String> {
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
