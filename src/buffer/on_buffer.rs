use bevy::prelude::Resource;

use super::{buffer_survey::BufferSurvey, Buffer};

pub fn on_buffer<B: Buffer + Resource>(reader: BufferSurvey<B>) -> bool {
    reader.survey().next().is_some()
}
