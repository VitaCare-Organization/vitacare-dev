import "./globals.css";
import Footer from "@/components/organisms/footer/Footer";

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
        {children}
        <Footer />
      </body>
    </html>
  );
}