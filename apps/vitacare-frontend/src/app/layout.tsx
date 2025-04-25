
import type { Metadata } from "next";
import "./globals.css";
import Footer from "@/components/organisms/Footer/Footer";
import Navbar from "@/components/organisms/Header/Header";

export const metadata: Metadata = {
  title: "VitaCare - Patient Registration",
  description: "Register to access the VitaCare platform",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <Navbar/>
        {children}
        <Footer />
      </body>
    </html>
  )
}
