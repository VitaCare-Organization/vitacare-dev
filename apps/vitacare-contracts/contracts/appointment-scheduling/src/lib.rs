#![no_std]
mod storage_types;
mod error;
mod contract;

pub use contract::AppointmentSchedulingContract;
pub use error::Error;
pub use storage_types::{Appointment, AppointmentStatus};

