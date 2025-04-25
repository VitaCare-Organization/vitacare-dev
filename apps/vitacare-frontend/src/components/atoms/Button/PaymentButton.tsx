import React from 'react';

interface PaymentButtonProps {
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
}

const PaymentButton: React.FC<PaymentButtonProps> = ({
  onClick,
  disabled = false,
  className = '',
  icon,
  children,
}) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`flex items-center px-4 py-2 cursor-pointer bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors ${disabled ? 'opacity-50 cursor-not-allowed' : ''} ${className}`}
    >
      {icon && <span className="mr-2">{icon}</span>}
      {children}
    </button>
  );
};

export default PaymentButton;