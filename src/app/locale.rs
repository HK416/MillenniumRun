use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};


lazy_static! {
    /// #### 한국어 </br>
    /// 각 로케일의 윈도우 제목 입니다. </br>
    /// 
    /// #### English (machine translation) </br>
    /// This is the window title for each locale. </br>
    /// 
    static ref TITLE_STR: HashMap<Locale, &'static str> = HashMap::from_iter([
        (Locale::Unknown, "Select your language."),
        (Locale::KOR, "밀레니엄 런"),
    ]);
}


/// #### 한국어  </br>
/// 어플리케이션의 로케일 목록 입니다. </br>
/// 
/// #### English (machine translation) </br>
/// This is a list of application locales. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[default]
    Unknown,
    KOR,
}

impl Locale {
    #[inline]
    pub fn is_unknown(&self) -> bool {
        match self {
            Self::Unknown => true,
            _ => false,
        }
    }
}



/// #### 한국어 </br>
/// 현재 로케일에 맞는 윈도우 제목을 가져옵니다. </br>
/// 윈도우 제목을 가져올 수 없는 경우 프로그램 실행을 중단시킵니다. </br>
/// 
/// ### English (machine translation) 
/// Get the window title appropriate for the current locale. </br>
/// If the window title cannot be retrieved, program execution will be stopped. </br>
/// 
#[inline]
pub fn get_wnd_title(locale: &Locale) -> &'static str {
    log::info!("locale: {:?}", locale);
    TITLE_STR.get(locale).expect("Unable to get window title for given locale. Please add the window title for the given locale.")
}
