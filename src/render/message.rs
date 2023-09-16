use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;


use crate::render::{
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
};



/// #### 한국어 </br>
/// 게임 로직 루프에서 게임 렌더 루프로 보내는 명령어 대기열 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the command queue sent from the game logic loop to the game render loop. </br>
/// 
static CMD_QUEUE: Mutex<VecDeque<(ResultSetter, RenderCommand)>> = Mutex::new(VecDeque::new());




/// #### 한국어 </br>
/// 게임 로직 루프에서 게임 랜더 루프로 명령어를 보내는 채널 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a channel that sends commands from the game logic loop to the game render loop. </br>
/// 
#[derive(Debug)]
pub struct RenderCommandChannel;

impl RenderCommandChannel {
    /// #### 한국어 </br>
    /// 명령 대기열에 명령어를 추가합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Add a command to the command queue. </br>
    /// 
    pub fn push(command: RenderCommand) -> ResultFuture {
        let val = Arc::new(Mutex::new(None));
        let cvar = Arc::new(Condvar::new());
        let setter = ResultSetter {
            val: val.clone(),
            cvar: cvar.clone(),
        };
        let future = ResultFuture {
            val: val.clone(),
            cvar: cvar.clone(),
        };

        CMD_QUEUE.lock()
            .expect("Failed to access render command queue.")
            .push_back((setter, command));
        return future;
    }

    /// #### 한국어 </br>
    /// 명령 대기열에 있는 오래된 명령어를 가져옵니다. </br>
    /// 만약 명령 대기열이 비어있는 경우 `None`을 반환합니다. </br>
    /// <b>메모: 이 함수는 게임 렌더 루프에서만 호출되어야 합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Fetch old commands from the command queue. </br>
    /// If the command queue is empty, it returns `None`. </br>
    /// <b>Note: This function must only be called from the game render loop.</b></br>
    /// 
    pub(super) fn pop() -> Option<(ResultSetter, RenderCommand)> {
        CMD_QUEUE.lock()
            .expect("Failed to access render command queue.")
            .pop_front()
    }
}



/// #### 한국어 </br>
/// 게임 로직 루프에서 게임 렌더 루프로 보낸 명령어의 결과를 설정하는 설정자 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a setter that sets the result of the command sent from the game logic loop to the game render loop. </br>
/// 
#[derive(Debug)]
pub struct ResultSetter {
    val: Arc<Mutex<Option<CommandResult>>>,
    cvar: Arc<Condvar>,
}

impl ResultSetter {
    /// #### 한국어 </br>
    /// 명령어 결과를 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set the command result. </br>
    /// 
    pub fn set(self, val: CommandResult) {
        *self.val.lock().expect("Failed to access command result.") = Some(val);
        self.cvar.notify_all();
    }
}



/// #### 한국어 </br>
/// 게임 로직 루프에서 게임 렌더 루프로 보낸 명령어의 결과를 저장합니다. </br>
/// 
/// #### English (Translation) </br>
/// Stores the results of commands sent from the game logic loop to the game render loop. </br>
/// 
#[derive(Debug)]
pub struct ResultFuture {
    val: Arc<Mutex<Option<CommandResult>>>,
    cvar: Arc<Condvar>,
}

impl ResultFuture {
    /// #### 한국어 </br>
    /// 명령어 결과를 기다립니다. </br>
    /// 이 함수는 게임 렌더 루프에서 처리를 완료할 때 까지 현재 스레드를 차단시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Wait for the command result. </br>
    /// This function blocks the current thread until the game render loop has completed processing. </br>
    /// 
    pub fn get_wait(self) -> CommandResult {
        let mut guard = self.cvar.wait_while(
            self.val.lock().expect("Failed to access command result."), 
            |val| val.is_none()
        ).expect("Failed to access command result.");
        guard.take().unwrap()
    }
} 



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
}
