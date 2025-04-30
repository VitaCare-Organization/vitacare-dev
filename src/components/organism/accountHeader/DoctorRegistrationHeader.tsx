
import React from 'react';
import logo from "../../../assets/SVG.svg";
import Image from "next/image"

interface RegistrationHeaderProps {
  title?: string;
  subtitle?: string;
}

const RegistrationHeader: React.FC<RegistrationHeaderProps> = ({ 
  title = "Create an account",
  subtitle = "Register to access the VitaCare platform"
}) => {
  return (
    <div className="text-center mb-6">
      <div className="flex justify-center mb-2">
         <Image src={logo} alt=""/>
      </div>
      <h2 className="text-2xl font-semibold text-gray-800">{title}</h2>
      <p className="text-gray-500 text-sm">{subtitle}</p>
    </div>
  );
};

export default RegistrationHeader;