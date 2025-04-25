"use client";

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
    <div className="min-h-screen flex justify-center items-center w-full">
      <div className="space-y-4 py-6 md:py-10 px-4 xl:py-0 xl:px-0 w-full">
        <div className="text-center flex flex-col gap-2">
          <Image
            className="mx-auto"
            src={badgeicon}
            alt="badge"
            width={20}
            height={20}
          />
          <h2 className="text-xl md:text-2xl leading-8 font-semibold text-[#020817]">
            Welcome back
          </h2>
          <p className="text-sm text-[#64748B] leading-5">
            Sign in to your VitaCare account
          </p>
        </div>
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="w-full md:max-w-[360px] md:mx-auto space-y-4 bg-white h-fit p-3 md:p-6 rounded-lg border border-[#E2E8F0] shadow-[0px_1px_2px_#0000000D]"
        >
          <div className="flex flex-col gap-1">
            <h2 className="font-semibold text-base md:text-2xl text-[#020817] leading-6">
              Sign In
            </h2>
            <p className="font-normal text-sm text-[#64748B] lg:w-5/6 leading-5">
              Enter your credentials to access your account
            </p>
          </div>
          <Input
            label="Email"
            placeholder="name@example.com"
            type="email"
            {...register("email")}
            error={errors.email?.message}
          />

          <div className="space-y-2">
            <PasswordInput
              label="Password"
              {...register("password")}
              error={errors.password?.message}
            />
          </div>

          <AccountTypeSelector
            setValue={setValue}
            selectedType={selectedAccountType}
          />
          {errors.accountType && (
            <p className="text-red-500 text-sm">{errors.accountType.message}</p>
          )}

          <div className="w-full flex justify-end">
            <a href="#" className="text-[#0096CC] text-sm hover:underline">
              Forgot password?
            </a>
          </div>

          <Button
            type="submit"
            fullWidth
            className="rounded-[6px] bg-[#0096CC] hover:brightness-110 py-3 text-[#18181B]"
          >
            <p className="text-sm leading-5 text-[#F8FAFC] font-medium">
              Sign In
            </p>
          </Button>

          <div className="flex justify-center text-sm items-center">
            <p className="text-[#A1A1AA]">
              <p>Don&apos;t have an account?</p>
              <Link className="text-sm hover:underline text-[#0096CC]" href="/">
                Sign up
              </Link>
            </p>
          </div>
        </form>
      </div>
    </div>
  );
};

export default LoginForm;
