import React from 'react';
import Image, { StaticImageData } from 'next/image';
import Link from 'next/link';

interface NavbarProps {
  logo: StaticImageData | string;
  activePage?: 'dashboard' | 'appointments' | 'records' | 'payments';
}

const Navbar: React.FC<NavbarProps> = ({ logo, activePage }) => {
  return (
    <nav className="bg-white border-b border-gray-200 px-4 sm:px-6 py-4">
      <div className="max-w-7xl mx-auto flex justify-between items-center">
        <div className="flex items-center space-x-8">
          <div className="flex items-center">
            <div className="text-blue-500 mr-2">
              <Image src={logo} alt="logo" width={40} height={40} />
            </div>
            <span className="font-bold text-xl">VitaCare</span>
          </div>
          <div className="hidden md:flex space-x-6">
            <Link href="/dashboard" className={`${activePage === 'dashboard' ? 'text-blue-500 font-medium' : 'text-gray-500'}`}>
              Dashboard
            </Link>
            <Link href="/appointments" className={`${activePage === 'appointments' ? 'text-blue-500 font-medium' : 'text-gray-500'}`}>
              Appointments
            </Link>
            <Link href="/records" className={`${activePage === 'records' ? 'text-blue-500 font-medium' : 'text-gray-500'}`}>
              Medical Records
            </Link>
            <Link href="/payments" className={`${activePage === 'payments' ? 'text-gray-500 font-medium' : 'text-gray-500'}`}>
              Payments
            </Link>
          </div>
        </div>
        <div className="flex items-center space-x-4">
          <button className="text-gray-500">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
              />
            </svg>
          </button>
          <div className="w-8 h-8 rounded-full bg-gray-200"></div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;