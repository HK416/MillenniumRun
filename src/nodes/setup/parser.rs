use std::env;

use crate::{
    components::user::Language,
    nodes::{
        first_time::FirstTimeSetupLoading,
        intro::IntroLoading,
        title::TitleLoading,
        in_game::InGameLoading,
    },
    scene::node::SceneNode,
};


const USAGE: &'static str = r#"
Usage: [PROGRAM_PATH] <OPTIONS>

Options
    -s <SCENE>, --scene <SCENE> : Specify the starting game scene. If no language is specified at this time, 'Korean' will be displayed.
    -l <LANGUAGE>, --language <LANGUAGE> : Specify the language used. This option does not affect the user settings file.

Scenes
    FirstTimeSetup, Intro, Title, InGame

Languages
    Korean
"#;


/// #### 한국어 </br>
/// 주어진 명령줄의 구문분석 결과를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// It contains the results of the syntax analysis of the given command line. </br>
/// 
#[derive(Debug)]
pub struct Config {
    pub next_scene: Option<Box<dyn SceneNode>>,
    pub language: Language,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self { 
            next_scene: None,
            language: Language::Korean,
        }
    }
}


/// #### 한국어 </br>
/// 주어진 명령줄을 구문분석 합니다. </br>
/// <b>잘못된 명령줄이 주어졌을 경우 프로그램 실행을 중단합니다.</b></br>
/// 
/// #### English (Translation)
/// The syntax analysis of the given command line. </br>
/// <b>If a wrong command line is given, it will stop running the program.<b></br>
/// 
pub fn parse_command_lines() -> Config {
    let mut config = Config::default();
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-s" | "--scene" => parse_scene_option(&mut iter, &mut config),
                "-l" | "--language" => parse_language_option(&mut iter, &mut config),
                _ => help(),
            }
        }
    }

    return config;
}


/// #### 한국어 </br>
/// 프로그램 사용 방법을 출력하고 프로그램 실행을 중단시킵니다. </br>
/// 
/// #### English (Translation) </br>
/// Prints out how to use the program and aborts the program running.
#[inline]
fn help() -> ! {
    panic!("{}", USAGE)
}


/// #### 한국어 </br>
/// 장면 옵션을 구문분석 합니다. </br>
/// 
/// #### English (Translation) </br>
/// Parses scene options. </br>
/// 
fn parse_scene_option<'a, I>(iter: &mut I, config: &mut Config)
where I: Iterator<Item = &'a String> {
    if let Some(arg)= iter.next() {
        match arg.as_str() {
            "FirstTimeSetup" => config.next_scene = Some(Box::new(FirstTimeSetupLoading::default())),
            "Intro" => config.next_scene = Some(Box::new(IntroLoading::default())),
            "Title" => config.next_scene = Some(Box::new(TitleLoading::default())),
            "InGame" => config.next_scene = Some(Box::new(InGameLoading::default())),
            _ => help()
        }
    } else {
        help()
    }
}


/// #### 한국어 </br>
/// 언어 옵션을 구문분석 합니다. </br>
/// 
/// #### English (Translation) </br>
/// Parses language options. </br>
/// 
fn parse_language_option<'a, I>(iter: &mut I, config: &mut Config) 
where I: Iterator<Item = &'a String> {
    if let Some(arg) = iter.next() {
        match arg.as_str() {
            "Korean" => config.language = Language::Korean,
            _ => help()
        }
    } else {
        help()
    }
}
