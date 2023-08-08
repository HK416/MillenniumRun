use std::include_str;
use std::path::PathBuf;
use std::collections::HashMap;
use lazy_static::lazy_static;
use super::AssetDataType;

lazy_static! {
    pub static ref ASSET_LIST: HashMap<PathBuf,AssetDataType> = {
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
    log::debug!("parsing AssetLists.txt");
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
            // 올바르지 않은 <유형>이거나, 이미 <경로>가 존재하는 경우 프로그램 실행을 중단시킵니다.
            // 
            // (English) 
            // If it has both <path> and <type>, put it in the list.
            // If <type> is invalid or <path> already exists, abort program execution.
            //
            if list.insert(
                PathBuf::from(collect_str[0].as_str()), 
                match collect_str[1].as_str() {
                    "Static" => AssetDataType::StaticData,
                    "Dynamic" => AssetDataType::DynamicData,
                    "Optional" => AssetDataType::OptionalData,
                    _ => { panic!("AssetLists.txt error: invalid types. (line:{})", num + 1) },
                }
            ).is_some() {
                panic!("AssetLists.txt error: duplicate path. (line:{})", num + 1);
            }
            log::debug!("Add AssetLists (PATH:{}, TYPE:{})", collect_str[0], collect_str[1]);
        }
        else {
            // (한국어) <경로>와 <유형>중 하나만 존재하는 경우 프로그램 실행을 중단시킵니다.
            // (English) Abort program execution if only one of <path> and <type> exists.
            panic!("AssetLists.txt error: invalid syntax. (line:{})", num + 1);
        }
    }

    return list;
}
