import FeatureCard from "@/components/molecules/Card/FeatureCard";
import { ShieldCheck, FileText,  Wallet, Shield, Calendar, Users } from "lucide-react";

const features = [
  { icon: <Users size={32} />, title: "Role-Based Access", description: "Tailored identities for hospitals, doctors, and patients with appropriate controls." },
  { icon: <FileText size={32} />, title: "Secure Medical Records", description: "Blockchain-verified medical history with secure sharing controls." },
  { icon: <Calendar size={32} />, title: "Appointment System", description: "Interactive scheduling with availability management and notifications." },
  { icon: <Wallet size={32} />, title: "Stellar Payments", description: "Integrated Stellar blockchain wallet for secure and transparent payments." },
  { icon: <Shield size={32} />, title: "Enhanced Security", description: "Verification-status indicators and encryption for maximum data protection." },
  { icon: <ShieldCheck size={32} />, title: "Blockchain Verification", description: "Clear indicators for blockchain-verified data and transactions." },
];

 

export default function FeatureGrid() {
  return (
    <div className="bg-[#F1F5F966] py-20 px-6 md:px-0 flex flex-col items-center justify-center">
        <h1 className="font-bold text-4xl mb-4">Key Features </h1>
        <p className="text-[#64748B] mb-10">Our platform provides comprehensive healthcare management with blockchain security.</p>
        <div className="w-[90%] md:w-[80%] lg:w-[65%] mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {features.map((feature, index) => (
            <FeatureCard key={index} {...feature} />
        ))}
        </div>
    </div>
  );
}
