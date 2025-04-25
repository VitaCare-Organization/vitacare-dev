import React from 'react';
import Image, { StaticImageData } from 'next/image';

interface ReceiptButtonProps {
  icon: StaticImageData | string;
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
}

const ReceiptButton: React.FC<ReceiptButtonProps> = ({
  icon,
  onClick,
  disabled = false,
  className = '',
  children,
}) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`text-gray-500 flex items-center border border-gray-200 px-3 py-2 rounded-lg gap-2 hover:bg-gray-50 transition-colors ${disabled ? 'opacity-50 cursor-not-allowed' : ''} ${className}`}
    >
      <Image src={icon} alt="icon" width={16} height={16} />
      {children}
    </button>
  );
};

export default ReceiptButton;