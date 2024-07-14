use bevy::{ecs::system::SystemParam, prelude::*};

use super::Buffer;

#[derive(SystemParam)]
pub struct BufferSurvey<'w, R: Buffer + Resource> {
    pub(super) buffer: Res<'w, R>,
}

impl<'a, R: Buffer + Resource> BufferSurvey<'a, R> {
    pub fn survey(&self) -> impl Iterator<Item = &R::Data> {
        self.buffer.survey()
    }
}
