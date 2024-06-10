use bevy::{ecs::system::SystemParam, prelude::*};

use super::Relay;

#[derive(SystemParam)]
pub struct RelaySurvey<'w, R: Relay> {
    relay: Res<'w, R>,
}

impl<'a, R: Relay> RelaySurvey<'a, R> {
    pub fn survey(&self) -> Option<&R::Data> {
        self.relay.survey()
    }
}
