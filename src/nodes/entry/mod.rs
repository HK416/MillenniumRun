mod ui;

use std::fmt;
use std::io::Cursor;
use std::sync::Arc;

use glam::DVec4;
use ab_glyph::FontArc;
use rand::{self, Rng};
use rodio::{Decoder, Sink};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text::{
            brush::TextBrush,
            section::{
                Align, 
                Section, 
                SectionBuilder,
            },
        },
        ui::{
            brush::UiBrush, 
            objects::UiObject
        }, 
        anchor::Anchor, 
        margin::Margin,
        sound::SoundDecoder, 
        user::{
            Language,
            Settings,
        },
    },
    nodes::{
        consts,
        path,
    },
    scene::{
        node::SceneNode, 
        state::SceneState,
    },
    system::{
        error::{
            AppResult,
            GameError,
        },
        shared::Shared,
    },
    render::{
        depth::DepthBuffer, 
        texture::DdsImageDecoder
    },
};


type UpdateFunc = &'static dyn Fn(&mut EntryScene, &mut Shared, f64, f64) -> AppResult<()>;
type DrawFunc = &'static dyn Fn(&EntryScene, &mut Shared) -> AppResult<()>;

const UPDATE_FUNC: [UpdateFunc; 6] = [
    &EntryScene::update_at_fade_in,
    &EntryScene::update_at_display_notify,
    &EntryScene::update_at_disappear_notify,
    &EntryScene::update_at_appear_logo,
    &EntryScene::update_at_display_logo,
    &EntryScene::update_at_disapper_logo,
];

const DRAW_FUNC: [DrawFunc; 6] = [
    &EntryScene::display_notify,
    &EntryScene::display_notify,
    &EntryScene::display_notify,
    &EntryScene::display_logo,
    &EntryScene::display_logo,
    &EntryScene::display_logo,
];



pub struct EntryScene {
    index: usize,
    elapsed_time: f64,
    notify_texts: Vec<Section>,
    background_color: DVec4,
    voice: Option<Decoder<Cursor<Vec<u8>>>>,
    logo_image: Option<ui::UiImage>,
}

impl EntryScene {
    fn update_at_fade_in(&mut self, _: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const DURATION: f64 = 0.5;
        self.elapsed_time += elapsed_time;
        if self.elapsed_time >= DURATION {
            self.index += 1;
            self.elapsed_time = 0.0;
            return Ok(());
        }

        let color = 1.0 * self.elapsed_time / DURATION;
        self.background_color = DVec4 { x: color, y: color, z: color, w: 1.0 };

        Ok(())
    }

    fn update_at_display_notify(&mut self, _: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const DURATION: f64 = 3.0;
        self.elapsed_time += elapsed_time;
        if self.elapsed_time >= DURATION {
            self.index += 1;
            self.elapsed_time = 0.0;
            return Ok(());
        }

        Ok(())
    }

    fn update_at_disappear_notify(&mut self, shared: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const DURATION: f64 = 0.5;
        self.elapsed_time += elapsed_time;
        if self.elapsed_time >= DURATION {
            self.index += 1;
            self.elapsed_time = 0.0;
            return Ok(());
        }

        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let alpha = 1.0 - 1.0 * (self.elapsed_time / DURATION) as f32;
        for button in self.notify_texts.iter_mut() {
            button.set_color_all((0.0, 0.0, 0.0, alpha).into());
            button.update(queue);
        }

        Ok(())
    }

    fn update_at_appear_logo(&mut self, shared: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const DURATION: f64 = 0.5;
        self.elapsed_time += elapsed_time;
        if self.elapsed_time >= DURATION {
            self.index += 1;
            self.elapsed_time = 0.0;
            return Ok(())
        }

        if let Some(voice) = self.voice.take() {
            let sink = shared.get::<Arc<Sink>>().unwrap();
            sink.append(voice);
        }

        let logo_image = self.logo_image.as_mut().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let alpha = 1.0 * (self.elapsed_time / DURATION) as f32;
        logo_image.data.color.w = alpha;
        logo_image.update_buffer(queue);
        
        Ok(())
    }

    fn update_at_display_logo(&mut self, _: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const DURATION: f64 = 3.0;
        self.elapsed_time += elapsed_time;
        if self.elapsed_time >= DURATION {
            self.index += 1;
            self.elapsed_time = 0.0;
            return Ok(())
        }

        Ok(())
    }

    fn update_at_disapper_logo(&mut self, shared: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const DURATION: f64 = 0.5;
        self.elapsed_time += elapsed_time;
        if self.elapsed_time >= DURATION {
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Pop;
            return Ok(());
        }

        let logo_image = self.logo_image.as_mut().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let alpha = 1.0 - 1.0 * (self.elapsed_time / DURATION) as f32;
        logo_image.data.color.w = alpha;
        logo_image.update_buffer(queue);

        Ok(())
    }

    fn display_notify(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let mut text_brush = shared.pop::<TextBrush>().unwrap();
        let font = shared.get::<FontArc>().unwrap();
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
        let sections: Vec<_> = self.notify_texts.iter().collect();


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
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Display Notify Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.background_color.x as f64,
                            g: self.background_color.y as f64,
                            b: self.background_color.z as f64,
                            a: self.background_color.w as f64,
                        }),
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
            
            text_brush.update_texture(font, device, queue, &sections);
            text_brush.draw(&sections, &mut rpass);
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        shared.push(text_brush);

        Ok(())
    }

    fn display_logo(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let ui_brush = shared.get::<UiBrush>().unwrap();
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
        let objects: [&dyn UiObject; 1] = [
            self.logo_image.as_ref().unwrap(),
        ];

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
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Display Logo Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.background_color.x as f64,
                            g: self.background_color.y as f64,
                            b: self.background_color.z as f64,
                            a: self.background_color.w as f64,
                        }),
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
            
            ui_brush.draw(&objects, &mut rpass);
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}

impl SceneNode for EntryScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let font = shared.get::<FontArc>().unwrap();
        let settings = shared.get::<Settings>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let ui_brush = shared.get::<UiBrush>().unwrap();
        
        // (한국어) 텍스트 생성하기.
        // (English Translation) Create a text.
        self.notify_texts.push(
            SectionBuilder::new(
                font, 
                56.0, 
                match settings.language {
                    Language::Unknown => panic!("Unknown locale!"),
                    Language::Korean => "알 림",
                }
            )
            .with_align(Align::Center((0.0, 2.5 * consts::PIXEL_PER_METER).into()))
            .build(device)
        );
        self.notify_texts.push(
            SectionBuilder::new(
                font, 
                42.0, 
                match settings.language {
                    Language::Unknown => panic!("Unknown locale!"),
                    Language::Korean => "이 게임은 Blue Archive의 팬 제작 게임입니다.",
                }
            )
            .with_align(Align::Center((0.0, 0.0).into()))
            .build(device)
        );

        // (한국어) 로고 텍스처를 생성합니다.
        // (English Translation) Create logo texture.
        let logo_texture = asset_bundle.get(path::LOGO_TEXTURE_PATH)?
            .read(&DdsImageDecoder::new(
                Some("Logo - Texture"),
                &device,
                &queue
            ))?;
        let logo_texture_view = logo_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let logo_tex_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Logo - Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                min_filter: wgpu::FilterMode::Linear,
                mag_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        );

        // (한국어) 로고 이미지 사용자 인터페이스를 생성합니다.
        // (English Translation) Create logo image user interface.
        let logo_image = ui::UiImageBuilder::new(
            Some("Logo"),
            &logo_tex_sampler,
            &logo_texture_view,
            ui_brush.ref_bind_group_layout()
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(120, -120, -120, 120))
        .with_color((1.0, 1.0, 1.0, 0.0).into())
        .build(device);

        /// (한국어) 캐릭터 타이틀 음성을 무작위로 선택합니다.
        /// (English Translation) Randomly selects character title voices.
        const VOICES: [&'static str; 4] = [
            path::YUZU_TITLE_SOUND_PATH,
            path::ARIS_TITLE_SOUND_PATH,
            path::MOMOI_TITLE_SOUND_PATH,
            path::MIDORI_TITLE_SOUND_PATH,
        ];
        let mut rng = rand::thread_rng();
        let voice = asset_bundle.get(VOICES[rng.gen_range(0..4)])?
            .read(&SoundDecoder)?;

        // (한국어) 사용을 완료한 에셋을 정리합니다.
        // (English Translation) Release assets that have been used.
        asset_bundle.release(path::LOGO_TEXTURE_PATH);
        for rel_path in VOICES {
            asset_bundle.release(rel_path);
        }

        self.voice = Some(voice);
        self.logo_image = Some(logo_image);

        Ok(())
    }

    #[inline]
    fn update(&mut self, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
        UPDATE_FUNC[self.index](self, shared, total_time, elapsed_time)
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        DRAW_FUNC[self.index](self, shared)
    }
}

impl Default for EntryScene {
    #[inline]
    fn default() -> Self {
        Self {
            index: 0,
            elapsed_time: 0.0,
            background_color: DVec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            notify_texts: Vec::new(),
            voice: None,
            logo_image: None,
        }
    }
}

impl fmt::Debug for EntryScene {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntryScene")
            .field("part", &self.index)
            .field("elapsed_time", &self.elapsed_time)
            .field("notify_texts", &self.notify_texts)
            .field("background_color", &self.background_color)
            .field("voice", &"random")
            .field("logo_image", &self.logo_image)
            .finish()
    }
}
