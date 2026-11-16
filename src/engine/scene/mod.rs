mod scene;
mod scene_manager;

use std::{cell::RefCell, rc::Rc};

//re-exports
pub use scene::Scene;
pub use scene_manager::SceneManager;

#[derive(Clone)]
pub enum Transition {
    None,
    Switch(Rc<RefCell<dyn Scene>>),
    Pause(Rc<RefCell<dyn Scene>>),
    Unpause,
    Quit,
}
