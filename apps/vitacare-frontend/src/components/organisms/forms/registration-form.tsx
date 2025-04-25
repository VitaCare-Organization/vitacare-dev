"use client"
import type React from "react"
import { useState } from "react"
import { Button } from "@/components/atoms/ui/button"
import FormField from "@/components/molecules/form-field"
import TabGroup from "@/components/molecules/tab-group"
import { Calendar } from "lucide-react"
import Link from "next/link"
import type { UserData } from "@/components/types"

export default function RegistrationForm() {
  const [formData, setFormData] = useState<UserData>({
    firstName: "",
    lastName: "",
    email: "",
    password: "",
    dateOfBirth: "",
  })

  const [isSubmitting, setIsSubmitting] = useState<boolean>(false)
  const [formError, setFormError] = useState<string | null>(null)
  const [formSuccess, setFormSuccess] = useState<boolean>(false)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target
    setFormData((prev: UserData) => ({
      ...prev,
      [name]: value,
    }))
  }

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setIsSubmitting(true)
    setFormError(null)

    try {
      if (!formData.firstName || !formData.lastName || !formData.email || !formData.password || !formData.dateOfBirth) {
        throw new Error("Please fill in all required fields")
      }

      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
      if (!emailRegex.test(formData.email)) {
        throw new Error("Please enter a valid email address")
      }

      if (formData.password.length < 8) {
        throw new Error("Password must be at least 8 characters long")
      }

      const dateRegex = /^(0[1-9]|1[0-2])\/(0[1-9]|[12]\d|3[01])\/(19|20)\d{2}$/
      if (!dateRegex.test(formData.dateOfBirth)) {
        throw new Error("Please enter a valid date in mm/dd/yyyy format")
      }
      console.log("Submitting form data:", formData)
      await new Promise((resolve) => setTimeout(resolve, 1000))
      setFormSuccess(true)
      setFormData({
        firstName: "",
        lastName: "",
        email: "",
        password: "",
        dateOfBirth: "",
      })
    } catch (error) {
      setFormError(error instanceof Error ? error.message : "An unexpected error occurred")
      console.error("Form submission error:", error)
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="space-y-6 max-w-[350px] m-auto">
      <TabGroup
        tabs={["Patient", "Doctor", "Hospital"]}
        defaultActiveTab="Patient"
        // onTabChange={(tab) => setActiveTab(tab as AccountType)}
      />

      <div className="pt-4">
        <h2 className="text-xl font-semibold text-gray-900">Patient Registration</h2>
        <p className="text-sm text-gray-500 mt-1">
          Create a patient account to manage your medical records and appointments.
        </p>
      </div>

      {formSuccess ? (
        <div className="bg-green-50 border border-green-200 rounded-md p-4 text-center">
          <h3 className="text-lg font-medium text-green-800">Registration Successful!</h3>
          <p className="text-green-700 mt-1">Your account has been created successfully.</p>
          <Button className="mt-4 bg-green-600 hover:bg-green-700 text-white" onClick={() => setFormSuccess(false)}>
            Register Another Account
          </Button>
        </div>
      ) : (
        <form onSubmit={handleSubmit} className="space-y-4">
          {formError && (
            <div className="bg-red-50 border border-red-200 rounded-md p-3 text-red-700 text-sm">{formError}</div>
          )}

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

          <Button type="submit" className="w-full bg-primary hover:bg-primary/90 text-white" disabled={isSubmitting}>
            {isSubmitting ? "Creating Account..." : "Create Patient Account"}
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
  )
}

