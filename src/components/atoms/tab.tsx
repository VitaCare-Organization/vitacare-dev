"use client"

import { cn } from "@/lib/utils"

interface TabProps {
  label: string
  isActive?: boolean
  onClick?: () => void
}

export default function Tab({ label, isActive = false, onClick }: TabProps) {
  return (
    <button
      className={cn(
        "px-4 py-2 m-1 text-sm font-medium  transition-colors sm:w-[115.49px]",
        isActive
          ? "bg-white text-gray-900 border-b-0 rounded"
          : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300",
      )}
      onClick={onClick}
    >
      {label}
    </button>
  )
}

