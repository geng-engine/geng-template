pub mod post;

use crate::{model::*, prelude::*};

pub const BACKGROUND_COLOR: Color = Color::BLACK;

#[allow(dead_code)]
pub struct GameRender {
    context: Context,
}

impl GameRender {
    pub fn new(context: &Context) -> Self {
        Self {
            context: context.clone(),
        }
    }

    pub fn draw(&mut self, _model: &Model, _framebuffer: &mut ugli::Framebuffer) {}
}
