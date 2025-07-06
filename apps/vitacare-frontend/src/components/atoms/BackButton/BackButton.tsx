import Link from "next/link"
import { ChevronLeft } from "lucide-react"

export default function BackButton() {
  return (
    <Link href="#" className="inline-flex items-center text-sm font-medium text-gray-800 hover:text-gray-600" data-testid="back-button">
      <ChevronLeft className="mr-1 h-4 w-4" />
      Back
    </Link>
  )
}

