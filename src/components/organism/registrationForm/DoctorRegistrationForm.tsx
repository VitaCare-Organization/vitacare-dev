// DoctorRegistrationForm.tsx
"use client";
import React from 'react';
import Link from 'next/link';
import { useForm } from 'react-hook-form';

type FormData = {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  licenseNumber: string;
  specialty: string;
};

type DoctorRegistrationFormProps = {
  onSubmit: (data: FormData) => void;
};

const DoctorRegistrationForm: React.FC<DoctorRegistrationFormProps> = ({ onSubmit }) => {
  const { 
    register, 
    handleSubmit, 
    formState: { errors } 
  } = useForm<FormData>({
    mode: 'onBlur'
  });

  return (
    <div>
      <h3 className="text-xl font-semibold text-gray-800 mb-1">Doctor Registration</h3>
      <p className="text-gray-500 text-sm mb-6">Create a doctor account to manage patients and appointments</p>

      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        {/* Name fields */}
        <div className="flex flex-col sm:flex-row gap-4">
          <div className="flex-1">
            <label htmlFor="firstName" className="block text-sm font-medium text-gray-700 mb-1">
              First name
            </label>
            <input
              id="firstName"
              placeholder="Jane"
              className={`w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-cyan-500 focus:border-cyan-500 ${errors.firstName ? 'border-red-500' : 'border-gray-300'}`}
              {...register('firstName', { 
                required: 'First name is required',
                minLength: { value: 2, message: 'Name must be at least 2 characters' }
              })}
            />
            {errors.firstName && (
              <p className="mt-1 text-xs text-red-600">{errors.firstName.message}</p>
            )}
          </div>
          <div className="flex-1">
            <label htmlFor="lastName" className="block text-sm font-medium text-gray-700 mb-1">
              Last name
            </label>
            <input
              id="lastName"
              placeholder="Smith"
              className={`w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-cyan-500 focus:border-cyan-500 ${errors.lastName ? 'border-red-500' : 'border-gray-300'}`}
              {...register('lastName', { 
                required: 'Last name is required',
                minLength: { value: 2, message: 'Name must be at least 2 characters' }
              })}
            />
            {errors.lastName && (
              <p className="mt-1 text-xs text-red-600">{errors.lastName.message}</p>
            )}
          </div>
        </div>

        {/* Email */}
        <div>
          <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-1">
            Email
          </label>
          <input
            type="email"
            id="email"
            placeholder="dr.smith@example.com"
            className={`w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-cyan-500 focus:border-cyan-500 ${errors.email ? 'border-red-500' : 'border-gray-300'}`}
            {...register('email', { 
              required: 'Email is required',
              pattern: {
                value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
                message: 'Invalid email address'
              }
            })}
          />
          {errors.email && (
            <p className="mt-1 text-xs text-red-600">{errors.email.message}</p>
          )}
        </div>

        {/* Password */}
        <div>
          <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-1">
            Password
          </label>
          <input
            type="password"
            id="password"
            className={`w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-cyan-500 focus:border-cyan-500 ${errors.password ? 'border-red-500' : 'border-gray-300'}`}
            {...register('password', { 
              required: 'Password is required',
              minLength: { value: 8, message: 'Password must be at least 8 characters' },
              pattern: {
                value: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/,
                message: 'Password must contain at least one uppercase letter, one lowercase letter, one number and one special character'
              }
            })}
          />
          {errors.password && (
            <p className="mt-1 text-xs text-red-600">{errors.password.message}</p>
          )}
        </div>

        {/* Medical License Number */}
        <div>
          <label htmlFor="licenseNumber" className="block text-sm font-medium text-gray-700 mb-1">
            Medical License Number
          </label>
          <input
            type="text"
            id="licenseNumber"
            placeholder="ML1234567B"
            className={`w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-cyan-500 focus:border-cyan-500 ${errors.licenseNumber ? 'border-red-500' : 'border-gray-300'}`}
            {...register('licenseNumber', { 
              required: 'Medical license number is required',
              pattern: {
                value: /^[A-Z0-9]+$/,
                message: 'License number should contain only letters and numbers'
              }
            })}
          />
          {errors.licenseNumber && (
            <p className="mt-1 text-xs text-red-600">{errors.licenseNumber.message}</p>
          )}
        </div>

        {/* Specialty */}
        <div>
          <label htmlFor="specialty" className="block text-sm font-medium text-gray-700 mb-1">
            Specialty
          </label>
          <input
            type="text"
            id="specialty"
            placeholder="Cardiology"
            className={`w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-cyan-500 focus:border-cyan-500 ${errors.specialty ? 'border-red-500' : 'border-gray-300'}`}
            {...register('specialty', { 
              required: 'Specialty is required'
            })}
          />
          {errors.specialty && (
            <p className="mt-1 text-xs text-red-600">{errors.specialty.message}</p>
          )}
        </div>

        {/* Submit button */}
        <div>
          <button
            type="submit"
            className="w-full bg-cyan-500 text-white py-2 px-4 rounded-md hover:bg-cyan-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-500"
          >
            Create Doctor Account
          </button>
        </div>
      </form>

      {/* Terms and Privacy */}
      <div className="mt-4 text-center text-xs text-gray-500">
        By clicking continue, you agree to our{' '}
        <Link href="/terms" className="text-cyan-600 hover:text-cyan-800">
          Terms of Service
        </Link>{' '}
        and{' '}
        <Link href="/privacy" className="text-cyan-600 hover:text-cyan-800">
          Privacy Policy
        </Link>
      </div>
    </div>
  );
};

export default DoctorRegistrationForm;