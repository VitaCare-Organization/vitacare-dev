import React from 'react';
import PayNowButton from '@/components/atoms/Button/PayNowButton';

type PendingPayment = {
  doctor: string;
  specialty: string;
  invoice: string;
  dueDate: string;
  amount: number;
};

interface PendingPaymentsProps {
  pendingPayments: PendingPayment[];
}

const PendingPayments: React.FC<PendingPaymentsProps> = ({ pendingPayments }) => {
  const handlePayNow = (payment: PendingPayment) => {
    console.log("Processing payment for:", payment);
    // ================Implement your payment processing logic here=========//
  };

  return (
    <div>
      <div className=" bg-white rounded-lg mt-4 shadow-sm border border-gray-200 p-6">
        <h2 className="text-2xl font-bold text-gray-800">Pending Payments</h2>
        <p className="text-sm text-gray-500 mb-6">Payments that require your attention</p>
      
      
      <div className="bg-white  rounded-lg border border-gray-200 overflow-h">
        {pendingPayments.map((payment, index) => (
          <div key={index} className="p-4 border-b last:border-b-0">
            <div className="flex leading-7 flex-col md:flex-row md:items-center md:justify-between">
              <div>
                <h3 className="font-medium text-gray-800">{payment.doctor} - {payment.specialty}</h3>
                <p className="text-sm text-gray-500">Invoice #{payment.invoice}</p>
                <p className="text-sm text-gray-500">Due: {payment.dueDate}</p>
              </div>
              <div className="mt-4 md:mt-0 flex items-center space-x-4">
                <span className="text-lg font-semibold">${payment.amount.toFixed(2)}</span>
                <PayNowButton onClick={() => handlePayNow(payment)} />
              </div>
            </div>
          </div>
        ))}
      </div>
      </div>
    </div>
  );
};

export default PendingPayments;