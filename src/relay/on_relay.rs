use bevy::prelude::Resource;

use super::{relay_survey::RelaySurvey, Relay};

pub fn on_relay<R: Relay + Resource>(reader: RelaySurvey<R>) -> bool {
    reader.survey().is_some()
}
