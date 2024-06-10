use super::{relay_survey::RelaySurvey, Relay};

pub fn on_relay<R: Relay>() -> impl FnMut(RelaySurvey<R>) -> bool + Clone {
    |reader: RelaySurvey<R>| reader.survey().is_some()
}
