use bevy::prelude::Resource;

use super::{buffer_handle::BufferHandle, Buffer};

pub fn buffer_cleanup<B: Buffer + Resource>(mut buffer: BufferHandle<B>) {
    buffer.clear();
}
