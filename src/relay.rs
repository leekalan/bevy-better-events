pub mod on_relay;
pub mod relay_cleanup;
pub mod relay_handle;
pub mod relay_survey;

pub mod basic_relay;
pub mod mut_relay;

pub trait Relay {
    type Data;

    fn post(&mut self, data: Self::Data) -> Result<(), Self::Data>;
    fn survey(&self) -> Option<&Self::Data>;
    fn mut_survey(&mut self) -> Option<&mut Self::Data>;
    fn receive(&mut self) -> Option<Self::Data>;
}
