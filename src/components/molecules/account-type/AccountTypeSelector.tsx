"use client";

import { UseFormSetValue } from "react-hook-form";
import { useState } from "react";

const accountTypes = ["Patient", "Doctor", "Hospital"];

interface AccountTypeSelectorProps {
  setValue: UseFormSetValue<any>;
  selectedType?: string;
}

export default function AccountTypeSelector({
  setValue,
  selectedType,
}: AccountTypeSelectorProps) {
  const [selected, setSelected] = useState(selectedType || "");

  const handleSelect = (type: string) => {
    setSelected(type);
    setValue("accountType", type, { shouldValidate: true });
  };

  return (
    <div className="flex flex-col gap-2">
      <h3 className="text-[#020817] text-sm font-medium">Account Type</h3>
      <div className="flex flex-col gap-2">
        {accountTypes.map((type) => (
          <label key={type} className="flex items-center gap-2 cursor-pointer">
            <input
              type="radio"
              name="accountType"
              value={type}
              checked={selected === type}
              onChange={() => handleSelect(type)}
              className="hidden"
            />
            <div
              className={`w-4 h-4 border border-[#0096CC] rounded-full flex items-center justify-center transition-all ${
                selected === type ? "bg-[#0096CC]" : "bg-white"
              }`}
            >
              {selected === type && (
                <div className="w-3 h-3 bg-white flex justify-center items-center rounded-full">
                  <div className="w-2 h-2 bg-[#0096CC] rounded-full"></div>
                </div>
              )}
            </div>
            <span className="text-[#020817] text-sm">{type}</span>
          </label>
        ))}
      </div>
    </div>
  );
}
