use thiserror::Error;

#[derive(Error, Debug)]
pub enum OfficeHoursError<'a> {
    #[error("Could not convert `{0:?}` to chrono::NaiveTime")]
    InvalidTimeSlice(&'a [u32]),
}
