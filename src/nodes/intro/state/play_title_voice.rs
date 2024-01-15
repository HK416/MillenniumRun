use std::thread;
use std::sync::Arc;

use rodio::OutputStreamHandle;
use rand::{self, Rng};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        camera::GameCamera,
        sound::SoundDecoder,
        user::Settings, 
    },
    nodes::intro::{IntroScene, state::IntroState},
    system::{
        error::{AppResult, GameError},
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// `intro` 게임 장면의 `PlayTitleVoice` 상태일 때 업데이트 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an update function when the `intro` game scene is in the `PlayTitleVoice` state. </br>
/// 
pub fn update(this: &mut IntroScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    use crate::{components::sound::play_sound, nodes::path};

    const NUM_CHARACTER: usize = 4;
    const VOICES: [&'static str; NUM_CHARACTER] = [
        path::ARIS_TITLE_SOUND_PATH,
        path::MOMOI_TITLE_SOUND_PATH,
        path::MIDORI_TITLE_SOUND_PATH,
        path::YUZU_TITLE_SOUND_PATH,
    ];
    
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 캐릭터 타이틀 음성을 무작위로 재생합니다.
    // (English Translation) Plays character title voices randomly.
    let mut rng = rand::thread_rng();
    let source = asset_bundle.get(VOICES[rng.gen_range(0..NUM_CHARACTER)])?
        .read(&SoundDecoder)?;
    let sink = play_sound(settings.voice_volume, source, stream)?;
    thread::spawn(move || {
        sink.sleep_until_end();
        sink.detach();
    });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    for rel_path in VOICES { 
        asset_bundle.release(rel_path) 
    };

    // (한국어) 다음 상태로 변경합니다.
    // (English Translation) Change to the next state.
    this.state = IntroState::AppearLogo;
    this.timer = 0.0;
    Ok(())
}



/// #### 한국어 </br>
/// `intro` 게임 장면의 `PlayTitleVoice` 상태일 때 그리기 함수 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a drawing function when the `intro` game scene is in the `PlayTitleVoice` state. </br>
/// 
pub fn draw(_this: &IntroScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();

    // (한국어) 이전 작업이 끝날 때 까지 기다립니다.
    // (English Translation) Wait until the previous operation is finished.
    device.poll(wgpu::Maintain::Wait);

    // (한국어) 다음 프레임을 가져옵니다.
    // (English Translation) Get the next frame.
    let frame = surface.get_current_texture()
        .map_err(|err| game_err!(
            "Failed to get next frame",
            "Failed to get next frame for the following reasons: {}",
            err.to_string()
        ))?;

    // (한국어) 프레임 버퍼의 텍스쳐 뷰를 생성합니다.
    // (English Translation) Creates a texture view of the framebuffer.
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(IntroScene(PlayTitleVoice(Ui)))"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                view: &view, 
                resolve_target: None, 
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        camera.bind(&mut rpass);
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
