import type { Metadata } from "next";
import "./globals.css";
import Footer from "@/components/organisms/Footer/footer";
import Navbar from "@/components/organisms/Header/Header";
import { UserProvider } from "@/context/userContext";

export const metadata: Metadata = {
  title: "VitaCare - Patient Registration",
  description: "Register to access the VitaCare platform",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <UserProvider>
          <Navbar />
          {children}
          <Footer />
        </UserProvider>
      </body>
    </html>
  );
}
