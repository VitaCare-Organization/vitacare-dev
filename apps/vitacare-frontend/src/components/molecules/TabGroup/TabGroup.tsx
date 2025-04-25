"use client"

import { useState } from "react"
import Tab from "@/components/atoms/Tab/Tab"

interface TabGroupProps {
  tabs: string[]
  defaultActiveTab?: string
  onTabChange?: (tab: string) => void
}

export default function TabGroup({ tabs, defaultActiveTab, onTabChange }: TabGroupProps) {
  const [activeTab, setActiveTab] = useState(defaultActiveTab || tabs[0])

  const handleTabChange = (tab: string) => {
    setActiveTab(tab)
    if (onTabChange) {
      onTabChange(tab)
    }
  }

  return (
    <div className="border-b border-gray-200 w-full">
      <div className="flex justify-between bg-[#F1F5F9]">
        {tabs.map((tab) => (
          <Tab key={tab} label={tab} isActive={activeTab === tab} onClick={() => handleTabChange(tab)} />
        ))}
      </div>
    </div>
  )
}

