// AccountTypeSelector.tsx
import React from 'react';
import Link from 'next/link';

type AccountType = 'patient' | 'doctor' | 'hospital';

interface AccountTypeSelectorProps {
  activeType: AccountType;
}

const AccountTypeSelector: React.FC<AccountTypeSelectorProps> = ({ activeType }) => {
  return (
    <div className="flex rounded-md shadow-sm mb-6">
      <Link 
        href="/patientRegistration" 
        className={`flex-1 py-2 text-center border border-gray-200 rounded-l-md ${
          activeType === 'patient' 
            ? 'bg-white text-gray-800 font-medium' 
            : 'text-gray-500 bg-gray-50 hover:bg-gray-100'
        }`}
      >
        Patient
      </Link>
      <Link 
        href="/doctorRegistration" 
        className={`flex-1 py-2 text-center border border-gray-200 ${
          activeType === 'doctor' 
            ? 'bg-white text-gray-800 font-medium' 
            : 'text-gray-500 bg-gray-50 hover:bg-gray-100'
        }`}
      >
        Doctor
      </Link>
      <Link 
        href="/hospitalRegistration" 
        className={`flex-1 py-2 text-center border border-gray-200 rounded-r-md ${
          activeType === 'hospital' 
            ? 'bg-white text-gray-800 font-medium' 
            : 'text-gray-500 bg-gray-50 hover:bg-gray-100'
        }`}
      >
        Hospital
      </Link>
    </div>
  );
};

export default AccountTypeSelector;