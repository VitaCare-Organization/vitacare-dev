import { forwardRef } from "react";

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  error?: string;
  id?: string;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, id, ...props }, ref) => {
    const inputId = id || label.toLowerCase().replace(/\s+/g, "-"); // Generate a unique ID

    return (
      <div className="flex flex-col">
        <label
          htmlFor={inputId}
          className="text-[#020817] font-medium text-xs sm:text-sm"
        >
          {label}
        </label>
        <input
          ref={ref}
          id={inputId}
          {...props}
          className="mt-1 p-2 border border-[#E2E8F0] rounded-[6px] 
          text-[#020817] focus:ring-2 focus:ring-slate-300"
        />
        {error && <p className="text-red-500 text-xs mt-1">{error}</p>}
      </div>
    );
  }
);

Input.displayName = "Input";

export default Input;
