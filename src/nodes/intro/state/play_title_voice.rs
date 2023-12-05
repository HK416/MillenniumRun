use std::thread;
use std::sync::Arc;

use rodio::OutputStreamHandle;
use rand::{self, Rng};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        sound::SoundDecoder,
        user::Settings, 
    },
    nodes::{path, intro::{IntroScene, state::IntroState}},
    render::depth::DepthBuffer,
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
    use crate::components::sound::create_sink;

    const NUM_CHARACTER: usize = 4;
    const VOICES: [&'static str; NUM_CHARACTER] = [
        path::intro::YUZU_SOUND_PATH,
        path::intro::ARIS_SOUND_PATH,
        path::intro::MOMOI_SOUND_PATH,
        path::intro::MIDORI_SOUND_PATH,
    ];
    
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let stream_handle = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 캐릭터 타이틀 음성을 무작위로 재생합니다.
    // (English Translation) Plays character title voices randomly.
    let mut rng = rand::thread_rng();
    let source = asset_bundle.get(VOICES[rng.gen_range(0..NUM_CHARACTER)])?
        .read(&SoundDecoder)?;
    let sink = create_sink(stream_handle)?;
    sink.set_volume(settings.voice_volume.get_norm());
    thread::spawn(move || {
        sink.append(source);
        sink.sleep_until_end();
    });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    for rel_path in VOICES { asset_bundle.release(rel_path) };

    // (한국어) 다음 상태로 변경합니다.
    // (English Translation) Change to the next state.
    this.state = IntroState::AppearLogo;
    this.elapsed_time = 0.0;
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
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();

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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(IntroScene(PlayTitleVoice(Ui)))"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                view: &view, 
                resolve_target: None, 
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                }
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth.view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
