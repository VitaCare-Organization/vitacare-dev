"use client";
import React, { useState } from "react";
import { PlusIcon, CreditCardIcon } from "@heroicons/react/24/outline";
import filterIcon from "@/assets/Frame.svg";
import logoImage from "@/assets/SVG.svg";
import PaymentButton from "@/components/atoms/Button/PaymentButton";
import ConnectWalletButton from "@/components/atoms/Button/ConnectWalletButton";
import FilterButton from "@/components/atoms/Button/FilterButton";
import CompletedPayments from "@/components/organism/payments/CompletedPayment";

import TabSwitcher from "@/components/molecules/Tapswitcher";
import PendingPayments from "@/components/organism/payments/PendingPayment";

type Payment = {
  doctor: string;
  specialty: string;
  invoiceNumber: string;
  date: string;
  amount: number;
};

// =========pending payment===========
type PendingPayment = {
  doctor: string;
  specialty: string;
  invoice: string;
  dueDate: string;
  amount: number;
};

const PaymentsDashboard: React.FC = () => {
  const handlePayment = () => {
    {
      /*======Payment logic here ==== */
    }

    console.log("Processing payment...");
  };

  const handleConnectWallet = () => {
    {
      /*======Connect Wallet logic here ==== */
    }
    console.log("Connecting wallet...");
  };
  const handleFilter = () => {
    /*======Filter logic here logic here ==== */
    console.log("Opening filter options...");
  };
  const handleShowReceipt = (payment: Payment) => {
    console.log("Show receipt for:", payment);
    // Implement your logic to show the receipt
  };

  const [activeTab, setActiveTab] = useState<string>("completed");

  const handleTabChange = (tabId: string) => {
    setActiveTab(tabId);
    /*========== Load different content based on the selected tab ======*/
    console.log(`Tab changed to: ${tabId}`);
  };

  const completedPayments: Payment[] = [
    {
      doctor: "Dr. Sarah Williams",
      specialty: "Dermatology Consultation",
      invoiceNumber: "INV-2025-0296",
      date: "February 15, 2025",
      amount: 85.0,
    },
    {
      doctor: "Dr. Robert Lee",
      specialty: "Dental Cleaning",
      invoiceNumber: "INV-2025-0275",
      date: "February 1, 2025",
      amount: 150.0,
    },
    {
      doctor: "Dr. Michael Chen",
      specialty: "Annual Physical",
      invoiceNumber: "INV-2025-0196",
      date: "January 15, 2025",
      amount: 200.0,
    },
  ];

  const pendingPayments: PendingPayment[] = [
    {
      doctor: "Dr. Jessica Brown",
      specialty: "Cardiology Consultation",
      invoice: "INV-2025-0312",
      dueDate: "March 30, 2025",
      amount: 120.0,
    },
  ];

  // =======Group payments by month ==========//
  const paymentsByMonth: Record<string, Payment[]> = {};
  completedPayments.forEach((payment) => {
    const month = payment.date.split(" ")[0];
    const year = payment.date.split(" ")[2];
    const key = `${month} ${year}`;

    if (!paymentsByMonth[key]) {
      paymentsByMonth[key] = [];
    }
    paymentsByMonth[key].push(payment);
  });

  return (
    <div className="min-h-screen bg-gray-50">

      {/* =============Main Content============== */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 py-6">
        <div className="flex flex-col md:flex-row md:items-center md:justify-between mb-6">
          <div>
            <h1 className="text-2xl font-bold text-gray-800">Payments</h1>
            <p className="text-gray-500 text-sm mt-1">
              Manage your healthcare payments securely on the Stellar blockchain
            </p>
          </div>

          <div className="flex mt-4 md:mt-0 space-x-2">
            {/* ======Payment Button ============ */}
            <PaymentButton
              onClick={handlePayment}
              icon={<CreditCardIcon className="h-5 w-5" />}
            >
              Make Payment
            </PaymentButton>
            {/* ======Wallet Connect Button ============ */}
            <ConnectWalletButton
              onClick={handleConnectWallet}
              icon={<PlusIcon className="h-5 w-5" />}
            >
              Connect Wallet
            </ConnectWalletButton>
          </div>
        </div>

        <div className=" mb-6">
          <div className="flex items-center justify-between py-6">
            <div className="relative  w-full">
              <input
                type="text"
                placeholder="Search payments..."
                className="w-full py-2 px-3 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </div>
            {/* ====== Filter Button ============ */}
            <FilterButton
              filterIcon={filterIcon}
              onClick={handleFilter}
              className="ml-4"
            >
              Filter
            </FilterButton>
          </div>

          {/* ========Tapswitcher component============= */}
          <div>
            <TabSwitcher
              tabs={[
                { id: "pending", label: "Pending" },
                { id: "completed", label: "Completed" },
                { id: "wallet", label: "Stellar Wallet" },
              ]}
              defaultActiveTab="completed"
              onChange={handleTabChange}
              className="w-[350px]"
            />

            {/* ========Render content based on Active tab =====   */}
            {activeTab === "pending" && (
              <PendingPayments pendingPayments={pendingPayments} />
            )}
            {activeTab === "completed" && (
              //  ==========Complete Payment Component===================//
              <CompletedPayments
                paymentsByMonth={paymentsByMonth}
                handleShowReceipt={handleShowReceipt}
              />
            )}
            {activeTab === "wallet" && <div>Stellar Wallet content here</div>}
          </div>
        </div>
      </div>
    </div>
  );
};

export default PaymentsDashboard;
