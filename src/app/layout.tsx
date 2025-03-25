import "./globals.css";
import Footer from "@/components/organisms/footer/Footer";
import Navbar from "@/components/organisms/header/Header";

export const metadata = {
  title: "VitaCare Health Portal",
  description: "Manage your healthcare payments securely",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Navbar/>
        {children}
        <Footer />
      </body>
    </html>
  );
}