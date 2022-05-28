use crate::error::{BoxDynError, Error};
use crate::sonyflake::{
    to_sonyflake_time, Internals, SharedSonyflake, Sonyflake, BIT_LEN_SEQUENCE,
};
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

/// A builder to build a [`Sonyflake`] generator.
///
/// [`Sonyflake`]: struct.Sonyflake.html
pub struct Builder<'a> {
    start_time: Option<DateTime<Utc>>,
    machine_id: Option<&'a dyn Fn() -> Result<u16, BoxDynError>>,
    check_machine_id: Option<&'a dyn Fn(u16) -> bool>,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Builder::new()
    }
}

impl<'a> Builder<'a> {
    /// Construct a new builder to call methods on for the [`Sonyflake`] construction.
    ///
    /// [`Sonyflake`]: struct.Sonyflake.html
    pub fn new() -> Self {
        Self {
            start_time: None,
            machine_id: None,
            check_machine_id: None,
        }
    }

    /// Sets the start time.
    /// If the time is ahead of current time, finalize will fail.
    pub fn start_time(mut self, start_time: DateTime<Utc>) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the machine id.
    /// If the fn returns an error, finalize will fail.
    pub fn machine_id(mut self, machine_id: &'a dyn Fn() -> Result<u16, BoxDynError>) -> Self {
        self.machine_id = Some(machine_id);
        self
    }

    /// Set a function to check the machine id.
    /// If the fn returns false, finalize will fail.
    pub fn check_machine_id(mut self, check_machine_id: &'a dyn Fn(u16) -> bool) -> Self {
        self.check_machine_id = Some(check_machine_id);
        self
    }

    /// Finalize the builder to create a Sonyflake.
    pub fn finalize(self) -> Result<Sonyflake, Error> {
        let sequence = 1 << (BIT_LEN_SEQUENCE - 1);

        let start_time = if let Some(start_time) = self.start_time {
            if start_time > Utc::now() {
                return Err(Error::StartTimeAheadOfCurrentTime(start_time));
            }

            to_sonyflake_time(start_time)
        } else {
            to_sonyflake_time(Utc.ymd(2014, 9, 1).and_hms(0, 0, 0))
        };

        let machine_id = if let Some(machine_id) = self.machine_id {
            match machine_id() {
                Ok(machine_id) => machine_id,
                Err(e) => return Err(Error::MachineIdFailed(e)),
            }
        } else {
            fake_machine_id()?
        };

        if let Some(check_machine_id) = self.check_machine_id {
            if !check_machine_id(machine_id) {
                return Err(Error::CheckMachineIdFailed);
            }
        }

        let shared = Arc::new(SharedSonyflake {
            internals: Mutex::new(Internals {
                sequence,
                elapsed_time: 0,
            }),
            start_time,
            machine_id,
        });
        Ok(Sonyflake::new_inner(shared))
    }
}

pub(crate) fn fake_machine_id() -> Result<u16, Error> {
    Ok(65535u16)
}
