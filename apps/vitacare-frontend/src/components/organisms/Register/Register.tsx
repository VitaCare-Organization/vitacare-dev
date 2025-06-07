import React from "react";
import BackButton from "@/components/atoms/BackButton/BackButton";
import AccountHeader from "../AccountHeader/AccountHeader";
import RegistrationForm from "../Forms/RegistrationForm/RegistrationForm";

const Register = () => {
  return (
    <div>
      <div className="min-h-screen flex items-center justify-center p-4">
        <div className="bg-white rounded-lg shadow-lg w-full max-w-4xl overflow-hidden">
          <div className="p-6 md:p-8">
            <div className="mb-8">
              <BackButton />
            </div>

            <div className="max-w-md mx-auto">
              <AccountHeader />
              <RegistrationForm />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Register;
