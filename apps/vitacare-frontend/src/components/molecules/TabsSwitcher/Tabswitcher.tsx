import React, { useState } from 'react';

type TabOption = {
  id: string;
  label: string;
};

interface TabSwitcherProps {
  tabs: TabOption[];
  defaultActiveTab?: string;
  onChange?: (tabId: string) => void;
  className?: string;
}

const TabSwitcher: React.FC<TabSwitcherProps> = ({
  tabs,
  defaultActiveTab,
  onChange,
  className = '',
}) => {
  const [activeTab, setActiveTab] = useState<string>(defaultActiveTab || tabs[0]?.id || '');

  const handleTabChange = (tabId: string) => {
    setActiveTab(tabId);
    if (onChange) {
      onChange(tabId);
    }
  };

  return (
    <div className={`flex border justify-center  border-gray-200 bg-gray-300 rounded-md ${className}`}>
      {tabs.map((tab) => (
        <button
          key={tab.id}
          className={`py-2 px-4 text-sm font-medium ${
            activeTab === tab.id
              ? "text-black bg-gray-100 rounded-md my-1 mx-1 border-black-600"
              : "text-gray-500"
          }`}
          onClick={() => handleTabChange(tab.id)}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
};

export default TabSwitcher;