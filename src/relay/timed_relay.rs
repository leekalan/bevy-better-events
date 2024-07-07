//! # TimedRelay
//!
//! `TimedRelay<R, T>` is a Relay that lasts through `T` cleanup cycles.
//!
//! Useful for simple communication between systems that do not always run

use bevy::prelude::*;

use crate::relay::Relay;

use super::{
    on_relay::on_relay, relay_cleanup::relay_cleanup, relay_handle::RelayHandle,
    relay_survey::RelaySurvey,
};

#[derive(Resource)]
pub struct TimedRelay<R: Relay, const T: u8> {
    relay: R,
    ticker: u8,
}

impl<R: Relay, const T: u8> TimedRelay<R, T> {
    pub fn ticker(&self) -> u8 {
        self.ticker
    }

    pub fn ticker_mut(&mut self) -> &mut u8 {
        &mut self.ticker
    }
}

impl<R: Relay + Default, const T: u8> Default for TimedRelay<R, T> {
    fn default() -> Self {
        Self {
            relay: R::default(),
            ticker: 0,
        }
    }
}

impl<R: Relay, const T: u8> Relay for TimedRelay<R, T> {
    type Data = R::Data;

    fn post(&mut self, data: Self::Data) -> Result<(), Self::Data> {
        self.relay.post(data).map(|_| self.ticker = T)
    }

    fn survey(&self) -> Option<&Self::Data> {
        self.relay.survey()
    }

    fn mut_survey(&mut self) -> Option<&mut Self::Data> {
        self.relay.mut_survey()
    }

    fn receive(&mut self) -> Option<Self::Data> {
        self.ticker = 0;
        self.relay.receive()
    }
}

pub type TimedRelayHandle<'w, R, const T: u8> = RelayHandle<'w, TimedRelay<R, T>>;

pub type TimedRelaySurvey<'w, R, const T: u8> = RelaySurvey<'w, TimedRelay<R, T>>;

impl<'w, R: Relay, const T: u8> TimedRelayHandle<'w, R, T>
where
    TimedRelay<R, T>: Resource,
{
    pub fn ticker(&self) -> u8 {
        self.relay.ticker
    }

    pub fn ticker_mut(&mut self) -> &mut u8 {
        &mut self.relay.ticker
    }
}

impl<'w, R: Relay, const T: u8> TimedRelaySurvey<'w, R, T>
where
    TimedRelay<R, T>: Resource,
{
    pub fn ticker(&self) -> u8 {
        self.relay.ticker
    }
}

pub fn timed_relay_cleanup<R: Relay, const T: u8>(mut relay: TimedRelayHandle<R, T>)
where
    TimedRelay<R, T>: Resource,
{
    if relay.relay.ticker == 0 {
        relay_cleanup(relay)
    } else {
        relay.relay.ticker -= 1
    }
}

pub fn on_timed_relay<R: Relay, const T: u8>(relay: TimedRelaySurvey<R, T>) -> bool
where
    TimedRelay<R, T>: Resource,
{
    on_relay(relay)
}

pub type LongRelay<R> = TimedRelay<R, 1>;
pub type LongRelayHandle<'w, R> = TimedRelayHandle<'w, R, 1>;
pub type LongRelaySurvey<'w, R> = TimedRelaySurvey<'w, R, 1>;

pub fn long_relay_cleanup<R: Relay>(relay: TimedRelayHandle<R, 1>)
where
    TimedRelay<R, 1>: Resource,
{
    relay_cleanup(relay)
}

pub fn on_long_relay<R: Relay>(relay: TimedRelaySurvey<R, 1>) -> bool
where
    TimedRelay<R, 1>: Resource,
{
    on_relay(relay)
}
