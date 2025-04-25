import React from 'react';
import Image, { StaticImageData } from 'next/image';

interface FilterButtonProps {
  filterIcon: StaticImageData | string;
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
}

const FilterButton: React.FC<FilterButtonProps> = ({
  filterIcon,
  onClick,
  disabled = false,
  className = '',
  children,
}) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`flex items-center border py-2 px-3 rounded-md border-gray-300 text-gray-700 gap-2 hover:bg-gray-50 transition-colors ${disabled ? 'opacity-50 cursor-not-allowed' : ''} ${className}`}
    >
      <Image src={filterIcon} alt="filter" width={20} height={20} />
      {children}
    </button>
  );
};

export default FilterButton;