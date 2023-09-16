use std::ops::Range;
use std::sync::Mutex;

use crate::render::{
    descriptor::RenderPassDesc,
    identifier::IDHandle,
};



/// #### 한국어 </br>
/// 제출된 그리기 명령을 저장합니다. </br>
/// 
/// #### English (Translation) </br>
/// Stores submitted drawing commands. </br>
/// 
static RENDER_SUBMIT: Mutex<Option<Vec<SubmitRenderPass>>> = Mutex::new(None);


/// #### 한국어 </br>
/// 그리기 명령을 전송하는 채널 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a channel that transmits drawing commands. </br>
/// 
#[derive(Debug)]
pub struct RenderSubmitChannel;

impl RenderSubmitChannel {
    /// #### 한국어 </br>
    /// 그리기 명령을 제출합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Submit a drawing command.
    /// 
    #[inline]
    pub fn upload(passes: Vec<SubmitRenderPass>) {
        *RENDER_SUBMIT.lock().expect("Failed to access render submit") = Some(passes);
    }

    /// #### 한국어 </br>
    /// 제출된 그리기 명령을 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the submitted drawing command. </br>
    /// 
    #[inline]
    pub(crate) fn load() -> Option<Vec<SubmitRenderPass>> {
        RENDER_SUBMIT.lock().expect("Failed to access render submit").clone()
    }
}



/// #### 한국어 </br>
/// 그리기 명령 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of drawing commands. </br>
/// 
#[derive(Debug, Clone)]
pub enum DrawCommand {
    SetPipeline {
        pipeline: IDHandle,
    },

    SetIndexBuffer {
        format: wgpu::IndexFormat,
        buffer: IDHandle,
        buffer_range: Range<wgpu::BufferAddress>,
    },

    SetVertexBuffer {
        slot: u32,
        buffer: IDHandle,
        buffer_range: Range<wgpu::BufferAddress>,
    },

    Draw { 
        vertices: Range<u32>, 
        instances: Range<u32>, 
    },

    DrawIndexed {
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    }
}


#[derive(Debug, Clone)]
pub struct SubmitRenderPass {
    pub desc: RenderPassDesc,
    pub commands: Vec<DrawCommand>,
}
