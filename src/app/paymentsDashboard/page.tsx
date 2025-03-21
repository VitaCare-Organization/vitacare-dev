"use client";

import React, { useState } from "react";
import Image from "next/image";
import logo from "@/assets/SVG.svg";
import arrow from "@/assets/Arrow.svg"
import {
  PlusIcon,
  CreditCardIcon,
} from "@heroicons/react/24/outline";

import filter from "@/assets/Frame.svg"

type Payment = {
  doctor: string;
  specialty: string;
  invoiceNumber: string;
  date: string;
  amount: number;
};

const PaymentsDashboard: React.FC = () => {
  const [activeTab, setActiveTab] = useState<
    "pending" | "completed" | "wallet"
  >("completed");

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
      {/* ========== Navbar========== */}
      <nav className="bg-white border-b border-gray-200 px-4 sm:px-6 py-4">
        <div className="max-w-7xl mx-auto flex justify-between items-center">
          <div className="flex items-center space-x-8">
            <div className="flex items-center">
              <div className="text-blue-500 mr-2">
                <Image src={logo} alt="logo" />
              </div>
              <span className="font-bold text-xl">VitaCare</span>
            </div>
            <div className="hidden md:flex space-x-6">
              <a href="#" className="text-gray-500">
                Dashboard
              </a>
              <a href="#" className="text-gray-500">
                Appointments
              </a>
              <a href="#" className="text-gray-500">
                Medical Records
              </a>
              <a href="#" className="text-gray-500 font-medium">
                Payments
              </a>
            </div>
          </div>
          <div className="flex items-center space-x-4">
            <button className="text-gray-500">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
                />
              </svg>
            </button>
            <div className="w-8 h-8 rounded-full bg-gray-200"></div>
          </div>
        </div>
      </nav>

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
            <button className="flex items-center px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors">
              <CreditCardIcon className="h-5 w-5 mr-2" />
              <span>Make Payment</span>
            </button>
            <button className="flex items-center px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors">
              <PlusIcon className="h-5 w-5 mr-2" />
              <span>Connect Wallet</span>
            </button>
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
            <button className="flex items-center border py-2 px-3  rounded-md border-gray-300 text-gray-700 gap-2 ml-4">
              <Image src={filter} alt="filter" />
              Filter
            </button>
          </div>

          <div className="flex border justify-center border-gray-200 w-[350px] bg-gray-300 rounded-md  ">
            <button
              className={`py-2 px-4 text-sm font-medium ${
                activeTab === "pending"
                  ? "text-black bg-gray-100 rounded-md my-1 mx-1 border-black-600"
                  : "text-gray-500"
              }`}
              onClick={() => setActiveTab("pending")}
            >
              Pending
            </button>
            <button
              className={`py-2 px-4 text-sm font-medium ${
                activeTab === "completed"
                  ? "text-black bg-gray-100 rounded-md my-1 mx-1 border-black"
                  : "text-gray-500"
              }`}
              onClick={() => setActiveTab("completed")}
            >
              Completed
            </button>
            <button
              className={`py-2 px-4 text-sm  font-medium ${
                activeTab === "wallet"
                  ? "text-black bg-gray-100 rounded-md my-1 mx-1  border-black"
                  : "text-gray-500"
              }`}
              onClick={() => setActiveTab("wallet")}
            >
              Stellar Wallet
            </button>
          </div>
        </div>

        {activeTab === "completed" && (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="text-2xl font-bold text-gray-800">
              Completed Payments
            </h2>
            <p className="text-sm text-gray-500 mb-6">Your payment history</p>

            {Object.entries(paymentsByMonth).map(([monthYear, payments]) => (
              <div key={monthYear} className="mb-8 border-2 rounded-xl border-gray-200">
                <h3 className="text-xl font-medium border-t border-gray-200 rounded-t-xl text-gray-800 bg-gray-100 p-4 mb-4">
                  {monthYear}
                </h3>

                <div className="space-y-4">
                  {payments.map((payment, index) => (
                    <div
                      key={index}
                      className="bg-white  border-t rounded-b-xl border-gray-200 p-4"
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
                          <button className="text-gray-500 flex items-center  border border-gray-200 px-3 py-2 rounded-lg gap-2 ">
                            <Image src={arrow} alt="arrow"  />
                            Receipt
                          </button>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}

        {activeTab === "pending" && (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="text-xl font-medium text-gray-800">
              Pending Payments
            </h2>
            <p className="text-sm text-gray-500 mb-6">
              Payments waiting to be processed
            </p>
            <div className="py-8 text-center text-gray-500">
              No pending payments at this time
            </div>
          </div>
        )}

        {activeTab === "wallet" && (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="text-xl font-medium text-gray-800">
              Stellar Wallet
            </h2>
            <p className="text-sm text-gray-500 mb-6">
              Manage your blockchain wallet
            </p>
            <div className="py-8 text-center text-gray-500">
              Please connect your Stellar wallet to view details
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default PaymentsDashboard;
