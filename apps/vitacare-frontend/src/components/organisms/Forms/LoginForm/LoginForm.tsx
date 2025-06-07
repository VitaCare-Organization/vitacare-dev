import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { LoginFormSchema } from "@/schemas/LoginFormSchema";
import { z } from "zod";
import Input from "@/components/atoms/Input/Input";
import { Button } from "@/components/atoms/Button/Button";
import Link from "next/link";
import PasswordInput from "@/components/atoms/Input/PasswordInput";
import AccountTypeSelector from "@/components/molecules/AccountType/AccountTypeSelector";
import Image from "next/image";
import badgeicon from "@/components/atoms/icons/badgeicon.svg";

type LoginFormData = z.infer<typeof LoginFormSchema>;

const LoginForm = () => {
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors },
  } = useForm<LoginFormData>({
    resolver: zodResolver(LoginFormSchema),
    mode: "onChange",
  });

  const selectedAccountType = watch("accountType");

  const onSubmit = (data: LoginFormData) => {
    if (!selectedAccountType) {
      alert("Please select an account type.");
      return;
    }
    console.log("Form Submitted:", data);
  };

  return (
    <div className="min-h-screen flex justify-center items-center w-full bg-gray-50">
      <div className="space-y-6 py-6 md:py-10 px-4 xl:py-0 xl:px-0 w-full max-w-md">
        {/* Header Section */}
        <div className="text-center flex flex-col gap-2">
          <Image
            className="mx-auto"
            src={badgeicon}
            alt="VitaCare Logo"
            width={24}
            height={24}
          />
          <h1 className="text-2xl font-semibold text-[#020817]">
            Welcome back
          </h1>
          <p className="text-sm text-[#64748B]">
            Sign in to your VitaCare account
          </p>
        </div>

        {/* Login Form */}
        <div className="bg-white rounded-lg border border-[#E2E8F0] shadow-sm p-6">
          <div className="mb-6">
            <h2 className="text-xl font-semibold text-[#020817] mb-1">
              Sign In
            </h2>
            <p className="text-sm text-[#64748B]">
              Enter your credentials to access your account
            </p>
          </div>

          <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
            {/* Email Input */}
            <Input
              label="Email"
              placeholder="name@example.com"
              type="email"
              {...register("email")}
              error={errors.email?.message}
            />

            {/* Password Input */}
            <PasswordInput
              label="Password"
              {...register("password")}
              error={errors.password?.message}
            />

            {/* Account Type Selector */}
            <div className="space-y-2">
              <AccountTypeSelector
                setValue={setValue}
                selectedType={selectedAccountType}
              />
              {errors.accountType && (
                <p className="text-red-500 text-sm">{errors.accountType.message}</p>
              )}
            </div>

            {/* Forgot Password Link */}
            <div className="flex justify-end">
              <Link 
                href="/forgot-password" 
                className="text-[#0096CC] text-sm hover:underline"
              >
                Forgot password?
              </Link>
            </div>

            {/* Submit Button */}
            <Button
              type="submit"
              className="w-full bg-[#0096CC] hover:bg-[#0096CC]/90 text-white py-3 rounded-md font-medium"
            >
              Sign In
            </Button>

            {/* Sign Up Link */}
            <div className="text-center text-sm">
              <span className="text-[#64748B]">Don&apos;t have an account? </span>
              <Link 
                href="/signup" 
                className="text-[#0096CC] hover:underline font-medium"
              >
                Sign up
              </Link>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
};

export default LoginForm;