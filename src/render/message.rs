
use crate::{
    panic_msg,
    app::abort::{PanicMsg, AppResult},
    render::{
        descriptor::{
            BindGroudLayoutDesc,
            BufferDesc,
            BufferInitDesc,
            PipelineLayoutDesc,
            ShaderModuleDesc,
            RenderPipelineDesc,
            CopyDesc,
        },
        identifier::IDHandle,
        task::SubmitRenderPass,
    }
};



/// #### 한국어 </br>
/// 명령어 처리 결과 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of command processing results. </br>
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    Failed,
    Finish,
    Return(IDHandle),
    QueryTextureFormat(wgpu::TextureFormat),
}

impl CommandResult {
    #[inline]
    pub fn finish_or_else<E, F>(self, err: F) -> Result<(), E>
    where F: FnOnce() -> E {
        match self {
            Self::Finish => Ok(()),
            _ => Err(err()),
        }
    }

    #[inline]
    pub fn return_or_else<E, F>(self, err: F) -> Result<IDHandle, E>
    where F: FnOnce() -> E {
        match self {
            Self::Return(id) => Ok(id),
            _ => Err(err()),
        }
    }

    #[inline]
    pub fn texture_format_or_else<E, F>(self, err: F) -> Result<wgpu::TextureFormat, E>
    where F: FnOnce() -> E {
        match self {
            Self::QueryTextureFormat(fmt) => Ok(fmt),
            _ => Err(err())
        }
    }
}



/// #### 한국어 </br>
/// 게임 로직 스레드에서 렌더 스레드로 보내는 명령어 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of commands sent from the game logic thread to the render thread. </br>
/// 
#[derive(Debug)]
pub enum RenderCommand {
    /// #### 한국어 </br>
    /// [BindGroupLayout](wgpu::BindGroupLayout)을 생성하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a command that creates [BindGroupLayout](wgpu::BindGroupLayout). </br>
    /// 
    CreateBindGroupLayout(BindGroudLayoutDesc),

    /// #### 한국어 </br>
    /// 비어있는 [Buffer](wgpu::Buffer)를 생성하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a command that creates empty [Buffer](wgpu::Buffer). </br>
    /// 
    CreateBuffer(BufferDesc),

    /// #### 한국어 </br>
    /// [Buffer](wgpu::Buffer)를 생성하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation)
    /// This is a command that creates [Buffer](wgpu::Buffer). </br>
    /// 
    CreateBufferWithData(BufferInitDesc),

    /// #### 한국어 </br>
    /// [PipelineLayout](wgpu::PipelineLayout)을 생성하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a command that creates [PipelineLayout](wgpu::PipelineLayout). </br>
    /// 
    CreatePipelineLayout(PipelineLayoutDesc),

    /// #### 한국어 </br>
    /// [ShaderModule](wgpu::ShaderModule)을 생성하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a command that creates [ShaderModule](wgpu::ShaderModule). </br>
    /// 
    CreateShaderModule(ShaderModuleDesc),

    /// #### 한국어 </br>
    /// [RenderPipeline](wgpu::RenderPipeline)을 생성하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a command that creates [RenderPipeline](wgpu::RenderPipeline). </br>
    /// 
    CreateRenderPipeline(RenderPipelineDesc),

    /// #### 한국어 </br>
    /// [Buffer](wgpu::Buffer)에서 [Buffer](wgpu::Buffer)로 복사하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a command that copy from [Buffer](wgpu::Buffer) to [Buffer](wgpu::Buffer). </br>
    /// 
    Copy(Vec<CopyDesc>),

    /// #### 한국어 </br>
    /// [Surface](wgpu::Surface)의 텍스쳐 포맷을 질의하는 명령어 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This command queries the texture format of [Surface](wgpu::Surface). </br>
    /// 
    QuerySwapchainFormat,

    Submit(Vec<SubmitRenderPass>),
}
