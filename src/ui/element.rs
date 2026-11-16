use std::path::PathBuf;

use crate::engine::scene::Transition;
use rapier3d::{
    glamx::{Vec2, vec2, vec4},
    math::Vec4,
};

pub enum Action {
    SceneTransition(Transition),
}

pub trait Element {
    fn get_texture(&self) -> &PathBuf;
    fn get_tris(&self) -> ([Vec2; 4], u32);
    fn set_first_idx(&mut self, value: u32);
    fn set_color(&mut self, value: Vec4);
    fn get_color(&self) -> Vec4;
    fn get_action(&self) -> &Action;
    fn in_bounds(&self, _position: Vec2) -> bool {
        false
    }
}

pub struct Rect {
    size: Vec2,
    origin: Vec2,
    first_index: u32,

    color: Vec4,

    action: Action,
    texture: PathBuf,
}

impl Rect {
    pub fn new(size: Vec2, origin: Vec2, action: Action, texture: PathBuf) -> Self {
        Self {
            size,
            origin,
            first_index: 0u32,

            color: vec4(1.0, 1.0, 1.0, 1.0),

            action,
            texture,
        }
    }
}

impl Element for Rect {
    fn get_action(&self) -> &Action {
        &self.action
    }

    fn get_texture(&self) -> &PathBuf {
        &self.texture
    }

    fn set_first_idx(&mut self, value: u32) {
        self.first_index = value;
    }

    fn get_tris(&self) -> ([Vec2; 4], u32) {
        let verts = [
            self.origin,
            vec2(self.origin.x + self.size.x, self.origin.y),
            self.origin + self.size,
            vec2(self.origin.x, self.origin.y + self.size.y),
        ];

        let first_index = self.first_index;

        (verts, first_index)
    }

    fn in_bounds(&self, position: Vec2) -> bool {
        let bottom_right = self.origin + self.size;

        position.cmpge(self.origin).all() && position.cmple(bottom_right).all()
    }

    fn set_color(&mut self, value: Vec4) {
        self.color = value;
    }

    fn get_color(&self) -> Vec4 {
        self.color
    }
}
