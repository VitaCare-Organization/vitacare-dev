
import React from "react";
import { Payment, PaymentsByMonth } from "@/types/Payment";
import ReceiptButton from "@/components/atoms/Button/ReceiptButton";
import arrowIcon from "@/assets/Arrow.svg";


interface CompletedPaymentsProps {
  paymentsByMonth: PaymentsByMonth;
  
  handleShowReceipt: (payment: Payment) => void;
}

const CompletedPayments: React.FC<CompletedPaymentsProps> = ({
  paymentsByMonth,
  handleShowReceipt,
}) => {
  return (
    <div className="bg-white rounded-lg mt-4 shadow-sm border border-gray-200 p-6">
      <h2 className="text-2xl font-bold text-gray-800">Completed Payments</h2>
      <p className="text-sm text-gray-500 mb-6">Your payment history</p>

      {Object.entries(paymentsByMonth).map(([monthYear, payments]) => (
        <div
          key={monthYear}
          className="mb-8 border-2 rounded-xl border-gray-200"
        >
          <h3 className="text-xl font-medium border-t border-gray-200 rounded-t-xl text-gray-800 bg-gray-100 p-4 mb-4">
            {monthYear}
          </h3>

          <div className="space-y-4">
            {payments.map((payment, index) => (
              <div
                key={index}
                className="bg-white border-t rounded-b-xl border-gray-200 p-4"
              >
                <div className="flex flex-col sm:flex-row leading-7 justify-between">
                  <div>
                    <h4 className="font-medium text-gray-800">
                      {payment.doctor} - {payment.specialty}
                    </h4>
                    <p className="text-sm text-gray-500">
                      Invoice #{payment.invoiceNumber}
                    </p>
                    <p className="text-sm text-gray-500">
                      Paid: {payment.date}
                    </p>
                  </div>
                  <div className="flex items-center mt-4 sm:mt-0">
                    <div className="flex items-center mr-4">
                      <span className="inline-flex items-center justify-center w-12 h-8 bg-green-100 rounded-full mr-2">
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          className="h-4 w-4 text-green-500"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M5 13l4 4L19 7"
                          />
                        </svg>
                      </span>
                      <span className="font-medium text-gray-800">
                        ${payment.amount.toFixed(2)}
                      </span>
                    </div>
                    <ReceiptButton icon={arrowIcon} onClick={() => handleShowReceipt(payment)}>
                      Receipt
                    </ReceiptButton>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
};

export default CompletedPayments;


