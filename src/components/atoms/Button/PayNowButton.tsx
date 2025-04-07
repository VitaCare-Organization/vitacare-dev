import React from 'react';

interface PayNowButtonProps {
  onClick: () => void;
  className?: string;
}

const PayNowButton: React.FC<PayNowButtonProps> = ({ onClick, className = '' }) => {
  return (
    <button 
      onClick={onClick}
      className={`px-4 py-1.5 bg-blue-500 text-white text-sm rounded hover:bg-blue-600 transition-colors ${className}`}
    >
      Pay Now
    </button>
  );
};

export default PayNowButton;