// types/Payment.ts
export interface Payment {
    doctor: string;
    specialty: string;
    invoiceNumber: string;
    date: string;
    amount: number;
  }
  
  export interface PaymentsByMonth {
    [monthYear: string]: Payment[];
  }