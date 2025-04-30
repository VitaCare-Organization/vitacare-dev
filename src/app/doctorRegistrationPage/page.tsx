// DoctorRegistrationPage.tsx
"use client"; 
import React from 'react';
import BackButton from "../../components/atoms/Button/BackButton";
import DoctorRegistrationForm from "../../components/organism/registrationForm/DoctorRegistrationForm";
import AccountTypeSelector from "../../components/molecules/account-type/DoctorAccountTypes";
import RegistrationHeader from "../../components/organism/accountHeader/DoctorRegistrationHeader";

type FormData = {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  licenseNumber: string;
  specialty: string;
};

const DoctorRegistrationPage = () => {
  const handleSubmit = (data: FormData) => {
    console.log(data);
   
  };

  return (
    <div className="min-h-screen bg-white flex flex-col">
      <div className="max-w-7xl mx-auto p-6 sm:px-6 lg:px-8 w-full">
        {/* Back button */}
        <BackButton/>
      </div>

      {/* Center the content */}
      <div className="flex-1 flex justify-center items-center py-8 px-4 sm:px-6 lg:px-8">
        <div className="w-full max-w-md">
          {/* Logo and title component */}
          <RegistrationHeader />

          {/* Account type selection component */}
          <AccountTypeSelector activeType="doctor" />

          {/* Registration form component */}
          <DoctorRegistrationForm onSubmit={handleSubmit} />
        </div>
      </div>
    </div>
  );
};

export default DoctorRegistrationPage;