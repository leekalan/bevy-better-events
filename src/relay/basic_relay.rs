//! # BasicRelay
//!
//! Basic relay that holds a posted value until it is recieved
//!
//! Useful for simple communication between systems

use bevy::prelude::*;

use super::{
    on_relay::on_relay, relay_cleanup::relay_cleanup, relay_handle::RelayHandle,
    relay_survey::RelaySurvey, Relay,
};

#[derive(Resource)]
pub struct BasicRelay<D> {
    data: Option<D>,
}

impl<D> Default for BasicRelay<D> {
    fn default() -> Self {
        Self { data: None }
    }
}

impl<D> Relay for BasicRelay<D> {
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

pub fn basic_relay_cleanup<D>(relay: BasicRelayHandle<D>)
where
    BasicRelay<D>: Resource,
{
    relay_cleanup(relay)
}

pub fn on_basic_relay<D>(relay: BasicRelaySurvey<D>) -> bool
where
    BasicRelay<D>: Resource,
{
    on_relay(relay)
}
