use soroban_sdk::{contracttype, Address};
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Appointments,
    NextAppointmentId,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum AppointmentStatus {
    Scheduled,
    Canceled,
    Completed,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Appointment {
    pub id: u64,
    pub patient: Address,
    pub doctor: Address,
    pub datetime: u64,
    pub status: AppointmentStatus,
}