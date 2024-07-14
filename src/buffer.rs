pub mod buffer_cleanup;
pub mod buffer_handle;
pub mod buffer_survey;
pub mod on_buffer;

pub mod basic_buffer;

pub trait Buffer {
    type Data;

    fn post(&mut self, data: Self::Data);
    fn survey<'s>(&'s self) -> impl Iterator<Item = &'s Self::Data>
    where
        Self::Data: 's;
    fn mut_survey<'s>(&'s mut self) -> impl Iterator<Item = &'s mut Self::Data>
    where
        Self::Data: 's;
    fn receive<'s, 'd>(&'s mut self) -> impl Iterator<Item = Self::Data> + 'd
    where
        Self::Data: 'd;
}
