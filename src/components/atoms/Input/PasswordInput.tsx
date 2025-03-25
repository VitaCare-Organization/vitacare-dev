import { useState } from "react";
import { Eye, EyeOff } from "lucide-react";

interface PasswordInputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  error?: string;
}

const PasswordInput: React.FC<PasswordInputProps> = ({
  label,
  error,
  id,
  ...props
}) => {
  const [showPassword, setShowPassword] = useState(false);
  const inputId = id || "password-input";

  return (
    <div className="flex flex-col space-y-1">
      <label
        htmlFor={inputId}
        className="text-[#020817] font-medium text-xs sm:text-sm"
      >
        {label}
      </label>
      <div className="relative">
        <input
          id={inputId}
          type={showPassword ? "text" : "password"}
          className="w-full p-2 pr-10 border border-[#E2E8F0] rounded-[6px]
           focus:ring-2 focus:ring-slate-300 text-[#020817]"
          {...props}
        />
        <button
          type="button"
          aria-label="toggle password visibility"
          onClick={() => setShowPassword((prev) => !prev)}
          className="absolute right-3 top-2 text-[#020817] cursor-pointer "
        >
          {showPassword ? (
            <EyeOff data-testid="eye-off-icon" size={18} />
          ) : (
            <Eye data-testid="eye-icon" size={18} />
          )}
        </button>
      </div>
      {error && <p className="text-red-500 text-xs">{error}</p>}
    </div>
  );
};

export default PasswordInput;
