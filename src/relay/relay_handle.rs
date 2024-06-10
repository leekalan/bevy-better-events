use bevy::{ecs::system::SystemParam, prelude::*};

use super::Relay;

#[derive(SystemParam)]
pub struct RelayHandle<'w, R: Relay> {
    relay: ResMut<'w, R>,
}

impl<'a, R: Relay> RelayHandle<'a, R> {
    pub fn post(&mut self, data: R::Data) -> Result<(), R::Data> {
        self.relay.post(data)
    }

    pub fn survey(&self) -> Option<&R::Data> {
        self.relay.survey()
    }

    pub fn mut_survey(&mut self) -> Option<&mut R::Data> {
        self.relay.mut_survey()
    }

    pub fn receive(&mut self) -> Option<R::Data> {
        self.relay.receive()
    }

    pub fn clear(&mut self) -> bool {
        self.receive().is_some()
    }
}
