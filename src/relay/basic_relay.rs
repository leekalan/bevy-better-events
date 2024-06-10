use bevy::prelude::*;

use super::{
    relay_cleanup::relay_cleanup, relay_handle::RelayHandle, relay_survey::RelaySurvey, Relay,
};

#[derive(Resource)]
pub struct BasicRelay<D: Resource> {
    data: Option<D>,
}

impl<D: Resource> Default for BasicRelay<D> {
    fn default() -> Self {
        Self { data: None }
    }
}

impl<D: Resource> Relay for BasicRelay<D> {
    type Data = D;

    fn post(&mut self, data: Self::Data) -> Result<(), Self::Data> {
        if self.data.is_none() {
            self.data = Some(data);
            Ok(())
        } else {
            Err(data)
        }
    }

    fn survey(&self) -> Option<&Self::Data> {
        self.data.as_ref()
    }

    fn mut_survey(&mut self) -> Option<&mut Self::Data> {
        self.data.as_mut()
    }

    fn receive(&mut self) -> Option<Self::Data> {
        self.data.take()
    }
}

pub type BasicRelayHandle<'w, D> = RelayHandle<'w, BasicRelay<D>>;

pub type BasicRelaySurvey<'w, D> = RelaySurvey<'w, BasicRelay<D>>;

pub fn basic_relay_cleanup<T: Resource>(relay: BasicRelayHandle<T>) {
    relay_cleanup::<BasicRelay<T>>(relay)
}
