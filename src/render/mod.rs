pub mod objects;
pub mod descriptor;
pub mod identifier;
pub mod initialize;
pub mod main_loop;
pub mod material;
pub mod message;
pub mod task;


pub mod types {
    use std::sync::mpsc::Sender;
    use crate::render::message::{
        CommandResult,
        RenderCommand,
    };

    pub type RenderCmdSenderType = Sender<(Sender<CommandResult>, RenderCommand)>;
}