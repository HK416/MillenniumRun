use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};


/// ### 한국어 
/// 어플리케이션에서 사용 가능한 장소 설정 목록입니다. </br>
/// 
/// ### English (machine translation)
/// A list of place settings available in the application. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[default]
    Unknown,
    EN,
    KOR,
}


/// ### 한국어 
/// 현재 장소 설정에 맞는 윈도우 제목을 가져옵니다. </br>
/// 
/// ### English (machine translation) 
/// Gets the window title that matches the current locale setting. </br>
/// 
#[inline]
pub fn get_wnd_title(locale: &Locale) -> &'static str {
    debug_assert!(WND_TITLE.get(locale).is_some(), "The window title does not exist. Please add a window title. (locale: {:?})", locale);
    unsafe { WND_TITLE.get(locale).unwrap_unchecked() }
}


lazy_static! {
    /// ### 한국어
    /// 각 장소 설정의 윈도우 제목 입니다. </br>
    /// 
    /// ### English (machine translation)
    /// Window title for each locale setting. </br>
    /// 
    static ref WND_TITLE: HashMap<Locale, &'static str> = HashMap::from([
        (Locale::Unknown, "Select your language."),
        (Locale::EN, "Millennium Run"),
        (Locale::KOR, "밀레니엄 런"),
    ]);
}
