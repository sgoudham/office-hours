//! Give your code a break like any good employer!
//!
//! Are you tired of your code working all day every day? Don't you feel bad that your code keeps working while you're off
//! relaxing and having fun after work?
//!
//! Well now you can use the power of **office-hours** to only run your code within the working hours of the day!
//!
//! # Important
//!
//! At the time of writing, the office hours are determined from the **Local Time Zone**
//! of the host machine where the code is running. I might consider updating this library
//! to support other timezones if I **really** want to suffer :P
//!
//! # Quick Start
//!
//! ```
//! use chrono::Timelike;
//! use office_hours::{Clock, OfficeHours};
//!
//! // 9am to 5pm are the default office hours
//! let office_hours = OfficeHours::default();
//! if office_hours.now() {
//!     println!("Blimey! Is it time for work already?");
//! } else {
//!     println!("Phew, still on break!");
//! }
//!
//! // 12pm to 5pm
//! let afternoon = OfficeHours::new(Clock::TwelvePm, Clock::FivePm);
//! if afternoon.now() {
//!     println!("Blimey! Is it time for work already?");
//! } else {
//!     println!("Phew, still on break!");
//! }
//!
//! // We can also cross the day boundary - 10pm to 6am
//! let night_shift = OfficeHours::new(Clock::TenPm, Clock::SixAm);
//! if night_shift.now() {
//!     println!("Blimey! Is it time for work already?");
//! } else {
//!     println!("Phew, still on break!");
//! }
//!
//! // Iterate over office hours as chrono::NaiveTime
//! for time in office_hours.iter() {
//!     println!("Hour: {:?}", time.hour());
//!     println!("Minute: {:?}", time.minute());
//!     println!("Second: {:?}", time.second());
//! }
//!
//! // Iterate over office hours as u32
//! for hour in office_hours.hours_iter() {
//!     println!("Hour: {:?}", hour);
//! }
//! ```
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::cmp::Ordering;

use chrono::{Duration, Local, NaiveTime, Timelike};

use thiserror::Error;
use OfficeHoursError::InvalidTimeSlice;

mod r#macro;

#[derive(Error, Debug)]
pub enum OfficeHoursError<'a> {
    #[error("Could not convert `{0:?}` to chrono::NaiveTime")]
    InvalidTimeSlice(&'a [u32]),
}

/// Simple enum to store all the valid hours in a day.
pub enum Clock {
    TwelveAm = 0,
    OneAm = 1,
    TwoAm = 2,
    ThreeAm = 3,
    FourAm = 4,
    FiveAm = 5,
    SixAm = 6,
    SevenAm = 7,
    EightAm = 8,
    NineAm = 9,
    TenAm = 10,
    ElevenAm = 11,
    TwelvePm = 12,
    OnePm = 13,
    TwoPm = 14,
    ThreePm = 15,
    FourPm = 16,
    FivePm = 17,
    SixPm = 18,
    SevenPm = 19,
    EightPm = 20,
    NinePm = 21,
    TenPm = 22,
    ElevenPm = 23,
}

/// Trait to implement helper functions in the style of [`From`].
pub trait FromNaiveTime {
    /// Makes a new [`NaiveTime`] out of a `&[u32]` slice.
    ///
    /// The slice supports a maximum length of 4 parts: `hour`,
    /// `minute`, `second`, and `millisecond`.
    ///
    /// If any part is missing, it is substituted with a `0`.
    ///
    /// If an empty slice is given, it will default
    /// to midnight.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if an invalid hour, minute, second,
    /// and/or millisecond.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveTime;
    /// use office_hours::FromNaiveTime;
    ///
    /// let midnight = NaiveTime::from_slice_u32(&[]).unwrap();
    /// assert_eq!(midnight, NaiveTime::default());
    ///
    /// let nine = NaiveTime::from_slice_u32(&[9]).unwrap();
    /// assert_eq!(nine, NaiveTime::from_hms_milli_opt(9, 0, 0, 0).unwrap());
    ///
    /// let nine_thirty = NaiveTime::from_slice_u32(&[9, 30]).unwrap();
    /// assert_eq!(
    ///     nine_thirty,
    ///     NaiveTime::from_hms_milli_opt(9, 30, 0, 0).unwrap()
    /// );
    /// ```
    ///
    /// See [`NaiveTime::from_hms_milli_opt`] for further information.
    fn from_slice_u32(slice: &[u32]) -> Result<NaiveTime, OfficeHoursError>;

    /// Makes a new [`NaiveTime`] out of a [`Clock`].
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveTime;
    /// use office_hours::{Clock, FromNaiveTime};
    /// let twelve_pm = NaiveTime::from_time(Clock::TwelvePm);
    /// assert_eq!(
    ///     twelve_pm,
    ///     NaiveTime::from_hms_milli_opt(12, 0, 0, 0).unwrap()
    /// )
    /// ```
    fn from_time(time: Clock) -> NaiveTime;

    /// Makes a new [`NaiveTime`] from the hour given as a [`u32`].
    ///
    /// # Errors
    ///
    /// Returns [`None`] when given an invalid hour.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveTime;
    /// use office_hours::FromNaiveTime;
    /// let twelve_pm = NaiveTime::from_time_u32(12).unwrap();
    /// assert_eq!(
    ///     twelve_pm,
    ///     NaiveTime::from_hms_milli_opt(12, 0, 0, 0).unwrap()
    /// )
    /// ```
    fn from_time_u32(hour: u32) -> Option<NaiveTime>;
}

impl FromNaiveTime for NaiveTime {
    fn from_slice_u32(slice: &[u32]) -> Result<NaiveTime, OfficeHoursError> {
        let time = Self::from_hms_milli_opt(
            *slice.first().unwrap_or(&0),
            *slice.get(1).unwrap_or(&0),
            *slice.get(2).unwrap_or(&0),
            *slice.get(3).unwrap_or(&0),
        )
        .ok_or(InvalidTimeSlice(slice))?;
        Ok(time)
    }

    fn from_time(hour: Clock) -> NaiveTime {
        // -- SAFETY --
        // Make sure all variants of the [Clock] enum continues
        // to hold hours of the day that valid (0 -> 23)
        unsafe { Self::from_time_u32(hour as u32).unwrap_unchecked() }
    }

    fn from_time_u32(hour: u32) -> Option<NaiveTime> {
        Self::from_hms_opt(hour, 0, 0)
    }
}

/// Simple struct to store the start and the finish
/// of the office day.
#[derive(Debug, Eq, PartialEq)]
pub struct OfficeHours {
    pub start: NaiveTime,
    pub finish: NaiveTime,
}

impl Default for OfficeHours {
    /// The default office hours are 9am to 5pm.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveTime;
    /// use office_hours::{Clock, FromNaiveTime, OfficeHours};
    ///
    /// let office_hours = OfficeHours::default();
    /// assert_eq!(office_hours.start, NaiveTime::from_time(Clock::NineAm));
    /// assert_eq!(office_hours.finish, NaiveTime::from_time(Clock::FivePm));
    /// ```
    fn default() -> Self {
        Self {
            start: NaiveTime::from_time(Clock::NineAm),
            finish: NaiveTime::from_time(Clock::FivePm),
        }
    }
}

impl OfficeHours {
    /// Makes a new [`OfficeHours`] from the starting and finishing time.
    ///
    /// # Examples
    ///
    /// ```
    /// use office_hours::{Clock, OfficeHours};
    /// let morning = OfficeHours::new(Clock::NineAm, Clock::TwelvePm);
    /// let afternoon = OfficeHours::new(Clock::TwelvePm, Clock::FivePm);
    /// ```
    #[must_use]
    pub fn new(start: Clock, finish: Clock) -> Self {
        Self {
            start: NaiveTime::from_time(start),
            finish: NaiveTime::from_time(finish),
        }
    }

    /// Returns an iterator over the office hours.
    ///
    /// The iterator yields all [`NaiveTime`] instances from
    /// the starting hour to finishing hour.
    ///
    /// # Examples
    ///
    /// ```
    /// use office_hours::{Clock, OfficeHours};
    ///
    /// let office_hours = OfficeHours::default();
    /// for time in office_hours.iter() {
    ///     println!("Hours: {:?}", time);
    /// }
    /// ```
    #[must_use]
    pub const fn iter(&self) -> OfficeHoursIter {
        OfficeHoursIter {
            start: self.start,
            finish: self.finish,
        }
    }

    /// Collect the contents of [`Self::hours_iter`] into a [`Vec<u32>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use office_hours::{Clock, OfficeHours};
    ///
    /// let office_hours = OfficeHours::default();
    /// assert_eq!(office_hours.hours(), vec![9, 10, 11, 12, 13, 14, 15, 16]);
    /// let night_shift = OfficeHours::new(Clock::TenPm, Clock::FiveAm);
    /// assert_eq!(night_shift.hours(), vec![22, 23, 0, 1, 2, 3, 4]);
    /// ```
    #[must_use]
    pub fn hours(&self) -> Vec<u32> {
        self.hours_iter().collect()
    }

    /// Returns an iterator over the office hours.
    ///
    /// The iterator yields all hours from
    /// the starting hour to finishing hour in its
    /// raw [`u32`] form.
    ///
    /// # Examples
    ///
    /// ```
    /// use office_hours::{Clock, OfficeHours};
    ///
    /// let office_hours = OfficeHours::default();
    /// for hour in office_hours.hours_iter() {
    ///     println!("u32: {:?}", hour);
    /// }
    /// ```
    pub fn hours_iter(&self) -> impl Iterator<Item = u32> {
        self.iter().map(|time| time.hour())
    }

    /// Checks if the current local time is within the office hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use office_hours::OfficeHours;
    ///
    /// let office_hours = OfficeHours::default();
    /// if office_hours.now() {
    ///     println!("Blimey! Is it time for work already?")
    /// } else {
    ///     println!("Phew, still on break!")
    /// }
    /// ```
    #[must_use]
    pub fn now(&self) -> bool {
        Self::now_from_time(&self.start, &self.finish, &Local::now().time())
    }

    #[doc(hidden)]
    #[must_use]
    pub fn now_from_time(start: &NaiveTime, finish: &NaiveTime, now: &NaiveTime) -> bool {
        match start.cmp(finish) {
            Ordering::Equal => start == now,
            Ordering::Less => (start..finish).contains(&now),
            Ordering::Greater => now >= start || now < finish,
        }
    }
}

impl<'a> TryFrom<(&'a [u32], &'a [u32])> for OfficeHours {
    type Error = OfficeHoursError<'a>;

    /// Performs the conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use office_hours::OfficeHours;
    ///
    /// let start: &[u32] = &[9, 30];
    /// let finish: &[u32] = &[17, 30];
    /// let valid_office_hours = OfficeHours::try_from((start, finish));
    /// assert!(valid_office_hours.is_ok());
    ///
    /// let start_err: &[u32] = &[100000000];
    /// let invalid_office_hours = OfficeHours::try_from((start_err, finish));
    /// assert!(invalid_office_hours.is_err());
    /// ```
    fn try_from(office_hours: (&'a [u32], &'a [u32])) -> Result<Self, Self::Error> {
        let (start, finish) = office_hours;
        Ok(Self {
            start: NaiveTime::from_slice_u32(start)?,
            finish: NaiveTime::from_slice_u32(finish)?,
        })
    }
}

/// Iterator over [`OfficeHours`] that returns the hourly [`NaiveTime`].
pub struct OfficeHoursIter {
    start: NaiveTime,
    finish: NaiveTime,
}

impl Iterator for OfficeHoursIter {
    type Item = NaiveTime;

    fn next(&mut self) -> Option<NaiveTime> {
        if self.start == self.finish {
            None
        } else {
            let current_hour = NaiveTime::from_time_u32(self.start.hour()).unwrap();
            let is_before_midnight = self.start == NaiveTime::from_time(Clock::ElevenPm);
            self.start = if is_before_midnight {
                NaiveTime::default()
            } else {
                self.start + Duration::hours(1)
            };
            Some(current_hour)
        }
    }
}

impl IntoIterator for &OfficeHours {
    type Item = NaiveTime;
    type IntoIter = OfficeHoursIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
