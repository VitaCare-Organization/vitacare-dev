import "./globals.css";

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
      </body>
    </html>
  );
}