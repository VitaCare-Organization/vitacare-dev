"use client"

import type React from "react"
import type { ReactNode } from "react"
import { Label } from "@/components/atoms/ui/label"
import { Input } from "@/components/atoms/ui/input"

interface FormFieldProps {
  label: string
  id: string
  type?: string
  placeholder?: string
  required?: boolean
  icon?: ReactNode
  value?: string
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void
}

export default function FormField({
  label,
  id,
  type = "text",
  placeholder = "",
  required = false,
  icon,
  value,
  onChange,
}: FormFieldProps) {
  return (
    <div className="space-y-2">
      <Label htmlFor={id} className="text-sm font-medium text-gray-700">
        {label}
      </Label>
      <div className="relative">
        <Input
          id={id}
          name={id}
          type={type}
          placeholder={placeholder}
          required={required}
          className="w-full border-gray-300 rounded-md"
          value={value}
          onChange={onChange}
        />
        {icon && <div className="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">{icon}</div>}
      </div>
    </div>
  )
}

