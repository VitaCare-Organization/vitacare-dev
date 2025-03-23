// Common type definitions for the project

export interface UserData {
    firstName: string
    lastName: string
    email: string
    password: string
    dateOfBirth: string
  }
  
  export type AccountType = "Patient" | "Doctor" | "Hospital"
  
  