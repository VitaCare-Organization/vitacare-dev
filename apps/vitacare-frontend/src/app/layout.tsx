import type { Metadata } from "next";
import Script from "next/script";
import "./globals.css";
import Footer from "@/components/organisms/Footer/footer";
import Navbar from "@/components/organisms/Header/header";
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
      <head>
        <Script
          src="https://www.googletagmanager.com/gtag/js?id=G-YXENNFY2TT"
          strategy="afterInteractive"
        />
        <Script id="google-analytics" strategy="afterInteractive">
          {`
            window.dataLayer = window.dataLayer || [];
            function gtag(){dataLayer.push(arguments);}
            gtag('js', new Date());
            gtag('config', 'G-YXENNFY2TT');
          `}
        </Script>
      </head>
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
