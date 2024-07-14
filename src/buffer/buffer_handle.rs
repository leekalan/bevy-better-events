use bevy::{ecs::system::SystemParam, prelude::*};

use super::Buffer;

#[derive(SystemParam)]
pub struct BufferHandle<'w, B: Buffer + Resource> {
    pub(super) buffer: ResMut<'w, B>,
}

impl<'a, B: Buffer + Resource> BufferHandle<'a, B> {
    pub fn post(&mut self, data: B::Data) {
        self.buffer.post(data)
    }

    pub fn survey(&self) -> impl Iterator<Item = &B::Data> {
        self.buffer.survey()
    }

    pub fn mut_survey(&mut self) -> impl Iterator<Item = &mut B::Data> {
        self.buffer.mut_survey()
    }

    pub fn receive(&mut self) -> impl Iterator<Item = B::Data> + '_ {
        self.buffer.receive()
    }

    pub fn clear(&mut self) -> bool {
        self.buffer.receive().next().is_some()
    }
}
