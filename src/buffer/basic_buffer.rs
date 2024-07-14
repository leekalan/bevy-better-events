//! # BasicBuffer
//!
//! Basic buffer that holds posted values until they are recieved
//!
//! Useful for building up events to be processed later

use bevy::prelude::*;

use super::{
    buffer_cleanup::buffer_cleanup, buffer_handle::BufferHandle, buffer_survey::BufferSurvey,
    on_buffer::on_buffer, Buffer,
};

#[derive(Resource)]
pub struct BasicBuffer<D> {
    buffer: Vec<D>,
}

impl<D> Default for BasicBuffer<D> {
    fn default() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl<D> Buffer for BasicBuffer<D> {
    type Data = D;

    fn post(&mut self, data: D) {
        self.buffer.push(data);
    }

    fn survey<'s>(&'s self) -> impl Iterator<Item = &'s D>
    where
        D: 's,
    {
        self.buffer.iter()
    }

    fn mut_survey<'s>(&'s mut self) -> impl Iterator<Item = &'s mut D>
    where
        D: 's,
    {
        self.buffer.iter_mut()
    }

    fn receive<'s, 'd>(&'s mut self) -> impl std::iter::Iterator<Item = D> + 'd
    where
        D: 'd,
    {
        let mut old = Vec::new();
        std::mem::swap(&mut self.buffer, &mut old);
        old.into_iter()
    }
}

pub type BasicBufferHandle<'w, D> = BufferHandle<'w, BasicBuffer<D>>;

pub type BasicBufferSurvey<'w, D> = BufferSurvey<'w, BasicBuffer<D>>;

pub fn basic_buffer_cleanup<D>(buffer: BasicBufferHandle<D>)
where
    BasicBuffer<D>: Resource,
{
    buffer_cleanup(buffer)
}

pub fn on_basic_buffer<D>(buffer: BasicBufferSurvey<D>) -> bool
where
    BasicBuffer<D>: Resource,
{
    on_buffer(buffer)
}
