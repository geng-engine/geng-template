mod logic;

use geng::prelude::*;

pub type FloatTime = R32;

pub struct Model {
    pub real_time: FloatTime,
}

impl Model {
    pub fn new() -> Self {
        Self {
            real_time: FloatTime::ZERO,
        }
    }
}
