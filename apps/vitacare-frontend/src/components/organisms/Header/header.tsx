
import React from 'react';
import Link from 'next/link';
import { Shield } from "lucide-react";

const Navbar: React.FC = () => {
  return (
    <header className="w-full border-b border-blue-100 px-8 md:px-20 py-4 md:py-3">
      <div className="container mx-auto px-4 py-4 flex items-center justify-between">

        <div className="flex items-center">
          <div className="text-blue-500 mr-2">
          <Shield className="text-[#0096CC]" />
          </div>
          <Link href="/" className="text-xl font-bold text-gray-800">
            VitaCare
          </Link>
        </div>

   
        <nav className="hidden md:flex items-center space-x-8">
          <Link href="/features" className="text-gray-600 hover:text-gray-900">
            Features
          </Link>
          <Link href="/security" className="text-gray-600 hover:text-gray-900">
            Security
          </Link>
          <Link href="/about" className="text-gray-600 hover:text-gray-900">
            About
          </Link>
        </nav>

      
        <div className="flex items-center space-x-4">
          <Link href="/login" className="text-gray-600 hover:text-gray-900">
            Log in
          </Link>
          <Link 
            href="/register" 
            className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors"
          >
            Register
          </Link>
        </div>
      </div>
    </header>
  );
};

export default Navbar;