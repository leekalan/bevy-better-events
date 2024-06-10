use super::{relay_handle::RelayHandle, Relay};

pub fn relay_cleanup<R: Relay>(mut relay: RelayHandle<R>) {
    relay.clear();
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::relay::basic_relay::{basic_relay_cleanup, BasicRelay, BasicRelayHandle};

    #[derive(Resource, Debug)]
    pub struct Count(pub i32);

    pub fn send(mut relay: BasicRelayHandle<Count>) {
        relay.post(Count(4)).unwrap();
    }

    pub fn receive_nothing(mut relay: BasicRelayHandle<Count>) {
        assert!(relay.receive().is_none());
    }

    pub fn receive_something(mut relay: BasicRelayHandle<Count>) {
        assert!(relay.receive().is_some());
    }

    #[test]
    fn test() {
        let mut app = App::new();

        app.init_resource::<BasicRelay<Count>>().add_systems(
            Update,
            (
                send,
                basic_relay_cleanup::<Count>,
                receive_nothing, // Receives no data as it was cleaned up by `basic_relay_cleanup``
                send,
                receive_something, // Receives the data sent by the second `send`
                basic_relay_cleanup::<Count>, // Does nothing, yet this is allowed as it is only cleanup
            )
                .chain(),
        );

        app.update();

        app.update()
    }
}
