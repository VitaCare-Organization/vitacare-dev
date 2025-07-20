"use client";
import type React from "react";
import { useState } from "react";
import { Button } from "@/components/atoms/ui/button";
import FormField from "@/components/molecules/FormField/formField";
import TabGroup from "@/components/molecules/TabGroup/tabGroup";
import { Calendar } from "lucide-react";
import Link from "next/link";

type AccountType = "Patient" | "Doctor" | "Hospital";

interface UserData {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  dateOfBirth: string;
  licenseNumber?: string;
  specialization?: string;
  hospitalName?: string;
  address?: string;
  hospitalLicense?: string;
}

export default function RegistrationForm() {
  const [activeTab, setActiveTab] = useState<AccountType>("Hospital");
  const [formData, setFormData] = useState<UserData>({
    firstName: "",
    lastName: "",
    email: "",
    password: "",
    dateOfBirth: "",
    hospitalLicense: "",
  });
  const [isSubmitting, setIsSubmitting] = useState<boolean>(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [formSuccess, setFormSuccess] = useState<boolean>(false);

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev: UserData) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleTabChange = (tab: string) => {
    setActiveTab(tab as AccountType);
    // Reset form when changing tabs
    setFormData({
      firstName: "",
      lastName: "",
      email: "",
      password: "",
      dateOfBirth: "",
      licenseNumber: "",
      specialization: "",
      hospitalName: "",
      address: "",
    });
    setFormError(null);
    setFormSuccess(false);
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);
    setFormError(null);

    try {
      if (
        !formData.firstName ||
        !formData.lastName ||
        !formData.email ||
        !formData.password ||
        !formData.dateOfBirth
      ) {
        throw new Error("Please fill in all required fields");
      }

      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
      if (!emailRegex.test(formData.email)) {
        throw new Error("Please enter a valid email address");
      }

      if (formData.password.length < 8) {
        throw new Error("Password must be at least 8 characters long");
      }

      const dateRegex =
        /^(0[1-9]|1[0-2])\/(0[1-9]|[12]\d|3[01])\/(19|20)\d{2}$/;
      if (!dateRegex.test(formData.dateOfBirth)) {
        throw new Error("Please enter a valid date in mm/dd/yyyy format");
      }

      // Additional validation based on account type
      if (activeTab === "Doctor") {
        if (!formData.licenseNumber || !formData.specialization) {
          throw new Error("Please fill in all doctor-specific fields");
        }
      }

      if (activeTab === "Hospital") {
        if (!formData.hospitalName || !formData.address) {
          throw new Error("Please fill in all hospital-specific fields");
        }
      }

      console.log("Submitting form data:", {
        accountType: activeTab,
        ...formData,
      });
      await new Promise((resolve) => setTimeout(resolve, 1000));
      setFormSuccess(true);
      setFormData({
        firstName: "",
        lastName: "",
        email: "",
        password: "",
        dateOfBirth: "",
        licenseNumber: "",
        specialization: "",
        hospitalName: "",
        address: "",
      });
    } catch (error) {
      setFormError(
        error instanceof Error ? error.message : "An unexpected error occurred"
      );
      console.error("Form submission error:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const renderFormFields = () => {
    switch (activeTab) {
      case "Doctor":
        return (
          <>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 outline-none">
              <FormField
                label="First name"
                id="firstName"
                placeholder="John"
                required
                value={formData.firstName}
                onChange={handleChange}
              />
              <FormField
                label="Last name"
                id="lastName"
                placeholder="Doe"
                required
                value={formData.lastName}
                onChange={handleChange}
              />
            </div>

            <FormField
              label="License Number"
              id="licenseNumber"
              placeholder="MD12345678"
              required
              value={formData.licenseNumber || ""}
              onChange={handleChange}
            />

            <FormField
              label="Specialization"
              id="specialization"
              placeholder="Cardiology"
              required
              value={formData.specialization || ""}
              onChange={handleChange}
            />

            <FormField
              label="Email"
              id="email"
              type="email"
              placeholder="john.doe@example.com"
              required
              value={formData.email}
              onChange={handleChange}
            />

            <FormField
              label="Password"
              id="password"
              type="password"
              required
              value={formData.password}
              onChange={handleChange}
            />

            <FormField
              label="Date of Birth"
              id="dateOfBirth"
              placeholder="mm/dd/yyyy"
              required
              icon={<Calendar className="h-5 w-5 text-gray-400" />}
              value={formData.dateOfBirth}
              onChange={handleChange}
            />
          </>
        );
      case "Hospital":
        return (
          <>
            <FormField
              label="Hospital Name"
              id="hospitalName"
              placeholder="City General Hospital"
              required
              value={formData.hospitalName || ""}
              onChange={handleChange}
            />
            <FormField
              label="Admin Email"
              id="email"
              type="email"
              placeholder="admin@cityhospital.com"
              required
              value={formData.email}
              onChange={handleChange}
            />

            <FormField
              label="Password"
              id="password"
              type="password"
              placeholder="admin Password"
              required
              value={formData.password}
              onChange={handleChange}
            />

            <FormField
              label="Hospital License Number"
              id="HpNum"
              type="text"
              placeholder="HL98765432"
              required
              value={formData.licenseNumber}
              onChange={handleChange}
            />
            <FormField
              label="Address"
              id="address"
              placeholder="123 Medical Center Blvd"
              required
              value={formData.address || ""}
              onChange={handleChange}
            />

            <FormField
              label="Phone Number"
              id="phoneNumber"
              placeholder="(555) 123-4567"
              required
              value={formData.address || ""}
              onChange={handleChange}
            />
          </>
        );
      case "Patient":
      default:
        return (
          <>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <FormField
                label="First name"
                id="firstName"
                placeholder="John"
                required
                value={formData.firstName}
                onChange={handleChange}
              />
              <FormField
                label="Last name"
                id="lastName"
                placeholder="Doe"
                required
                value={formData.lastName}
                onChange={handleChange}
              />
            </div>

            <FormField
              label="Email"
              id="email"
              type="email"
              placeholder="john.doe@example.com"
              required
              value={formData.email}
              onChange={handleChange}
            />

            <FormField
              label="Password"
              id="password"
              type="password"
              required
              value={formData.password}
              onChange={handleChange}
            />

            <FormField
              label="Date of Birth"
              id="dateOfBirth"
              placeholder="mm/dd/yyyy"
              required
              icon={<Calendar className="h-5 w-5 text-gray-400" />}
              value={formData.dateOfBirth}
              onChange={handleChange}
            />
          </>
        );
    }
  };

  const getFormTitle = () => {
    switch (activeTab) {
      case "Doctor":
        return {
          title: "Doctor Registration",
          description:
            "Create a doctor account to manage your practice and patient records.",
        };
      case "Hospital":
        return {
          title: "Hospital Registration",
          description:
            "Create a hospital account to manage your facility and staff.",
        };
      case "Patient":
      default:
        return {
          title: "Patient Registration",
          description:
            "Create a patient account to manage your medical records and appointments.",
        };
    }
  };

  return (
    <div className="space-y-6 max-w-[350px] m-auto">
      <TabGroup
        tabs={["Patient", "Doctor", "Hospital"]}
        defaultActiveTab="Patient"
        onTabChange={handleTabChange}
      />

      <div className="pt-4">
        <h2 className="text-xl font-semibold text-gray-900">
          {getFormTitle().title}
        </h2>
        <p className="text-sm text-gray-500 mt-1">
          {getFormTitle().description}
        </p>
      </div>

      {formSuccess ? (
        <div className="bg-green-50 border border-green-200 rounded-md p-4 text-center">
          <h3 className="text-lg font-medium text-green-800">
            Registration Successful!
          </h3>
          <p className="text-green-700 mt-1">
            Your {activeTab.toLowerCase()} account has been created
            successfully.
          </p>
          <Button
            className="mt-4 bg-green-600 hover:bg-green-700 text-white"
            onClick={() => setFormSuccess(false)}
          >
            Register Another Account
          </Button>
        </div>
      ) : (
        <form onSubmit={handleSubmit} className="space-y-4">
          {formError && (
            <div className="bg-red-50 border border-red-200 rounded-md p-3 text-red-700 text-sm">
              {formError}
            </div>
          )}

          {renderFormFields()}

          <Button
            type="submit"
            className="w-full bg-primary hover:bg-primary/90 text-white"
            disabled={isSubmitting}
          >
            {isSubmitting
              ? `Creating ${activeTab} Account...`
              : `Create ${activeTab} Account`}
          </Button>
        </form>
      )}

      <div className="text-xs text-center text-gray-500">
        By clicking continue, you agree to our{" "}
        <Link href="#" className="text-gray-700 underline">
          Terms of Service
        </Link>{" "}
        and{" "}
        <Link href="#" className="text-gray-700 underline">
          Privacy Policy
        </Link>
        .
      </div>
    </div>
  );
}
