use soroban_sdk::{
    contract, contractimpl,
    log, map, vec,
    Address, Env, Map, Vec,
};

use super::error::Error;
use super::storage_types::{Appointment, AppointmentStatus, DataKey};

#[contract]
pub struct AppointmentSchedulingContract;

#[contractimpl]
impl AppointmentSchedulingContract {

    pub fn initialize(env: Env) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::NextAppointmentId) {
             return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::NextAppointmentId, &1u64);
        let appointments: Map<u64, Appointment> = map![&env];
        env.storage().persistent().set(&DataKey::Appointments, &appointments);
        log!(&env, "Contract initialized. Appointment counter set to 1.");

        Ok(())
    }

    pub fn create_appointment(
        env: Env,
        patient: Address,
        doctor: Address,
        datetime: u64,
    ) -> Result<u64, Error> {
        patient.require_auth();

        let current_time = env.ledger().timestamp();
        if datetime <= current_time {
             return Err(Error::PastTimestamp);
        }

        let next_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextAppointmentId)
            .ok_or(Error::NotInitialized)?;

        let mut appointments: Map<u64, Appointment> = env
            .storage()
            .persistent()
            .get(&DataKey::Appointments)
            .ok_or(Error::NotInitialized)?;

        let appointment_id = next_id;
        let new_appointment = Appointment {
            id: appointment_id,
            patient: patient.clone(),
            doctor: doctor.clone(),
            datetime,
            status: AppointmentStatus::Scheduled,
        };

        appointments.set(appointment_id, new_appointment);
    
        env.storage().persistent().set(&DataKey::Appointments, &appointments);

        env.storage().instance().set(&DataKey::NextAppointmentId, &(next_id + 1));

        log!(&env, "Appointment created: ID={}, Patient={}, Doctor={}, Time={}",
            appointment_id, patient, doctor, datetime);

        Ok(appointment_id)
    }


    pub fn cancel_appointment(env: Env, appointment_id: u64) -> Result<(), Error> {
        let mut appointments: Map<u64, Appointment> = env
            .storage()
            .persistent()
            .get(&DataKey::Appointments)
            .ok_or(Error::NotInitialized)?;

        let mut appointment = appointments
            .get(appointment_id)
            .ok_or(Error::AppointmentNotFound)?;
        appointment.patient.require_auth();

        if appointment.status != AppointmentStatus::Scheduled {
            return Err(Error::InvalidStatus);
        }

        appointment.status = AppointmentStatus::Canceled;

        appointments.set(appointment_id, appointment.clone());
        env.storage().persistent().set(&DataKey::Appointments, &appointments);

        log!(&env, "Appointment canceled: ID={}", appointment_id);
        Ok(())
    }

    pub fn complete_appointment(env: Env, appointment_id: u64) -> Result<(), Error> {
         let mut appointments: Map<u64, Appointment> = env
            .storage()
            .persistent()
            .get(&DataKey::Appointments)
            .ok_or(Error::NotInitialized)?;

        let mut appointment = appointments
            .get(appointment_id)
            .ok_or(Error::AppointmentNotFound)?;

        appointment.doctor.require_auth();
        if appointment.status != AppointmentStatus::Scheduled {
             return Err(Error::InvalidStatus);
        }
        appointment.status = AppointmentStatus::Completed;


        appointments.set(appointment_id, appointment.clone());
        env.storage().persistent().set(&DataKey::Appointments, &appointments);

        log!(&env, "Appointment completed: ID={}", appointment_id);
         Ok(())
    }

    pub fn get_appointments(env: Env, user: Address) -> Vec<Appointment> {
        let appointments: Map<u64, Appointment> = env
            .storage()
            .persistent()
            .get(&DataKey::Appointments)
            .unwrap_or_else(|| map![&env]);

        let mut user_appointments: Vec<Appointment> = vec![&env];

        for (_id, appointment) in appointments.iter() {
            if appointment.patient == user || appointment.doctor == user {
                user_appointments.push_back(appointment);
            }
        }
        user_appointments
    }

    pub fn get_appointment_details(env: Env, appointment_id: u64) -> Result<Appointment, Error> {
         let appointments: Map<u64, Appointment> = env
            .storage()
            .persistent()
            .get(&DataKey::Appointments)
            .ok_or(Error::NotInitialized)?;
        appointments.get(appointment_id).ok_or(Error::AppointmentNotFound)
    }
}