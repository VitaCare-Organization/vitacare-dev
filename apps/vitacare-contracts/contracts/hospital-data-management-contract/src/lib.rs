#![no_std]

mod hospital;
mod access_control;
mod utils;

pub use hospital::{Hospital, HospitalContract, HospitalContractClient};

#[cfg(test)]
mod test;
