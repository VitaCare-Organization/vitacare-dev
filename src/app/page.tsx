import Link from "next/link";

export default function Home() {
  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <h1 className="text-2xl font-bold mb-6">VitaCare Portal</h1>
      
      <div className="flex flex-col space-y-4">
        <Link 
          href="/paymentsDashboard" 
          className="p-4 bg-white rounded-lg border border-gray-200 hover:shadow-md transition-shadow"
        >
          <h2 className="text-xl font-semibold text-blue-600">Payments Dashboard</h2>
          <p className="text-gray-600">Manage your healthcare payments</p>
        </Link>
      </div>
    </div>
  );
}
