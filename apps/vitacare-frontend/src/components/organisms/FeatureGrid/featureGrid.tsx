import React from 'react';
import { Users, FileText, Calendar, CreditCard, Shield, CheckCircle } from 'lucide-react';

interface FeatureItem {
  icon: React.ReactNode;
  title: string;
  description: string;
}

const KeyFeaturesSection: React.FC = () => {
  const features: FeatureItem[] = [
    {
      icon: <Users className="w-8 h-8 text-blue-500" />,
      title: "Role-Based Access",
      description: "Tailored interfaces for hospitals, doctors, and patients with appropriate controls."
    },
    {
      icon: <FileText className="w-8 h-8 text-blue-500" />,
      title: "Secure Medical Records",
      description: "Blockchain-verified medical history with secure sharing controls."
    },
    {
      icon: <Calendar className="w-8 h-8 text-blue-500" />,
      title: "Appointment System",
      description: "Smart scheduling with availability management and notifications."
    },
    {
      icon: <CreditCard className="w-8 h-8 text-blue-500" />,
      title: "Stellar Payments",
      description: "Verified Stellar blockchain wallet for secure and transparent payments."
    },
    {
      icon: <Shield className="w-8 h-8 text-blue-500" />,
      title: "Enhanced Security",
      description: "Multi-layered using biometrics and encryption for maximum data protection."
    },
    {
      icon: <CheckCircle className="w-8 h-8 text-blue-500" />,
      title: "Blockchain Verification",
      description: "Stellar indicators for blockchain-verified data and transactions."
    }
  ];

  return (
    <section className="py-16 px-4 sm:px-6 lg:px-8 bg-gray-50">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="text-center mb-12">
          <h2 className="text-4xl sm:text-4xl font-bold text-gray-900 mb-4">
            Key Features
          </h2>
          <p className="text-xl text-gray-600 max-w-3xl mx-auto">
            Our platform provides comprehensive healthcare management with blockchain security.
          </p>
        </div>

        {/* Features Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <div
              key={index}
              className="bg-white rounded-xl p-8 shadow-sm hover:shadow-md transition-shadow duration-300 border border-gray-100"
            >
              {/* Icon */}
              <div className="flex justify-center mb-6">
                <div className="p-3 bg-blue-50 rounded-lg">
                  {feature.icon}
                </div>
              </div>

              {/* Content */}
              <div className="text-center">
                <h3 className="text-2xl font-semibold text-gray-900 mb-3">
                  {feature.title}
                </h3>
                <p className="text-gray-600 leading-relaxed text-xl">
                  {feature.description}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default KeyFeaturesSection;