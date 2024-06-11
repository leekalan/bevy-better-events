use bevy::prelude::*;

use super::{
    relay_cleanup::relay_cleanup, relay_handle::RelayHandle, relay_survey::RelaySurvey, Relay,
};

#[derive(Resource)]
pub struct MutRelay<D> {
    data: Option<D>,
    mutator: Option<fn(&mut D)>,
}

impl<D> MutRelay<D> {
    pub fn insert_mutator(&mut self, mutator: fn(&mut D)) -> Result<(), fn(&mut D)> {
        if self.mutator.is_none() {
            self.mutator = Some(mutator);
            Ok(())
        } else {
            Err(mutator)
        }
    }

    pub fn take_mutator(&mut self) -> Option<fn(&mut D)> {
        self.mutator.take()
    }

    pub fn clear_mutator(&mut self) {
        self.take_mutator();
    }
}

impl<D> Default for MutRelay<D> {
    fn default() -> Self {
        Self {
            data: None,
            mutator: None,
        }
    }
}

impl<D> Relay for MutRelay<D> {
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

pub type MutRelayHandle<'w, D> = RelayHandle<'w, MutRelay<D>>;

impl<'w, D: Send + Sync + 'static> MutRelayHandle<'w, D> {
    pub fn insert_mutator(&mut self, mutator: fn(&mut D)) -> Result<(), fn(&mut D)> {
        self.relay.insert_mutator(mutator)
    }

    pub fn take_mutator(&mut self) -> Option<fn(&mut D)> {
        self.relay.take_mutator()
    }

    pub fn clear_mutator(&mut self) {
        self.relay.clear_mutator();
    }
}

pub type MutRelaySurvey<'w, D> = RelaySurvey<'w, MutRelay<D>>;

pub fn basic_relay_cleanup<D>(relay: MutRelayHandle<D>)
where
    MutRelay<D>: Resource,
{
    relay_cleanup::<MutRelay<D>>(relay)
}

pub fn on_basic_relay<D>() -> impl FnMut(MutRelaySurvey<D>) -> bool + Clone
where
    MutRelay<D>: Resource,
{
    |reader: MutRelaySurvey<D>| reader.survey().is_some()
}
