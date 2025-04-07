import React from 'react';

interface ConnectWalletButtonProps {
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
}

const ConnectWalletButton: React.FC<ConnectWalletButtonProps> = ({
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
      className={`flex items-center px-4 py-2 border cursor-pointer border-gray-300 rounded-md hover:bg-gray-50 transition-colors ${disabled ? 'opacity-50 cursor-not-allowed' : ''} ${className}`}
    >
      {icon && <span className="mr-2">{icon}</span>}
      {children}
    </button>
  );
};

export default ConnectWalletButton;