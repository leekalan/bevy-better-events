//! # DelayedRelay
//!
//! `DelayedRelay<R>` is a Relay that hides the posted value from everything until it is enabled
//!
//! Useful for a relay that is meant to be surveyed by multiple systems

use bevy::prelude::*;

use crate::relay::Relay;

use super::{
    on_relay::on_relay, relay_cleanup::relay_cleanup, relay_handle::RelayHandle,
    relay_survey::RelaySurvey,
};

#[derive(Resource)]
pub struct DelayedRelay<R: Relay> {
    relay: R,
    enabled: bool,
}

impl<R: Relay> DelayedRelay<R> {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

impl<R: Relay + Default> Default for DelayedRelay<R> {
    fn default() -> Self {
        Self {
            relay: R::default(),
            enabled: false,
        }
    }
}

impl<R: Relay> Relay for DelayedRelay<R> {
    type Data = R::Data;

    fn post(&mut self, data: Self::Data) -> Result<(), Self::Data> {
        self.relay.post(data).map(|_| self.enabled = false)
    }

    fn survey(&self) -> Option<&Self::Data> {
        if self.enabled {
            self.relay.survey()
        } else {
            None
        }
    }

    fn mut_survey(&mut self) -> Option<&mut Self::Data> {
        if self.enabled {
            self.relay.mut_survey()
        } else {
            None
        }
    }

    fn receive(&mut self) -> Option<Self::Data> {
        if self.enabled {
            self.enabled = false;
            self.relay.receive()
        } else {
            None
        }
    }
}

pub type DelayedRelayHandle<'w, R> = RelayHandle<'w, DelayedRelay<R>>;

pub type DelayedRelaySurvey<'w, R> = RelaySurvey<'w, DelayedRelay<R>>;

impl<'w, R: Relay> DelayedRelayHandle<'w, R>
where
    DelayedRelay<R>: Resource,
{
    pub fn enabled(&self) -> bool {
        self.relay.enabled
    }

    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.relay.enabled
    }

    pub fn enable(&mut self) {
        self.relay.enabled = true
    }
}

impl<'w, R: Relay> DelayedRelaySurvey<'w, R>
where
    DelayedRelay<R>: Resource,
{
    pub fn enabled(&self) -> bool {
        self.relay.enabled
    }
}

pub fn delayed_relay_cleanup<R: Relay>(relay: DelayedRelayHandle<R>)
where
    DelayedRelay<R>: Resource,
{
    relay_cleanup(relay) // Internally uses receive so it won't delete unless enabled
}

pub fn on_delayed_relay<R: Relay>(relay: DelayedRelaySurvey<R>) -> bool
where
    DelayedRelay<R>: Resource,
{
    on_relay(relay) // Internally uses survey so it won't return true unless enabled
}

pub fn enable_delayed_relay<R: Relay>(mut relay: DelayedRelayHandle<R>)
where
    DelayedRelay<R>: Resource,
{
    relay.relay.enabled = true
}
