use std::{cell::RefCell, rc::Rc};

use super::{Scene, Transition};
use crate::engine::Engine;

#[cfg(feature = "imgui")]
use crate::engine::Debug;

pub struct SceneManager {
    current: Rc<RefCell<dyn Scene>>,
    overlay: Option<Rc<RefCell<dyn Scene>>>,
}
impl SceneManager {
    pub fn new(initial: Rc<RefCell<dyn Scene>>, engine: &mut Engine) -> Self {
        initial.borrow_mut().on_enter(engine);

        SceneManager {
            current: initial,
            overlay: None,
        }
    }

    pub fn handle_input(&mut self, engine: &mut Engine) {
        self.current.borrow_mut().handle_input(engine);
    }

    pub fn update(&mut self, dt: f32, engine: &mut Engine) -> bool {
        let current = self.current.clone();

        let result = current.borrow_mut().update(dt, engine);

        match result {
            Transition::None => true,
            Transition::Switch(next) => {
                current.borrow_mut().on_exit(engine);
                next.borrow_mut().on_enter(engine);
                self.current = next;
                true
            }
            Transition::Pause(overlay) => {
                overlay.borrow_mut().on_enter(engine);
                self.overlay = Some(overlay.clone());
                true
            }
            Transition::Unpause => {
                self.overlay.as_mut().unwrap().borrow_mut().on_exit(engine);
                self.overlay = None;
                current.borrow_mut().on_unpause(engine);
                true
            }
            Transition::Quit => false,
        }
    }

    pub fn render(&mut self, engine: &mut Engine) {
        let mut frame = engine.start_frame();

        self.current.borrow_mut().render(engine, &mut frame);

        match self.overlay.as_mut() {
            None => {}
            Some(scene) => scene.borrow_mut().render_as_overlay(engine, &mut frame),
        }

        engine.finish_frame(frame);
    }

    #[cfg(feature = "imgui")]
    pub fn render_debug(&mut self, engine: &mut Engine, debug: &mut Debug) {
        self.current.borrow_mut().render_debug(engine, debug);
    }
}
