mod scene;
mod main_loop;
mod object;


pub use self::{
    main_loop::game_logic_loop,
    object::GameObject,
    scene::Scene,
};
