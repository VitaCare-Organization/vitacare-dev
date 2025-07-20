import React from "react";

const StellerPayment = () => {
  const transactions = [
    {
      date: "February 15, 2025",
      recipient: "City General Hospital",
      amount: "-$85.00",
      tx: "TX: STLR_AFRD",
    },
    {
      date: "February 1, 2025",
      recipient: "Bright Smile Dental",
      amount: "-$150.00",
      tx: "TX: STLR_BASIC",
    },
    {
      date: "January 25, 2025",
      recipient: "From Bank Account",
      amount: "+$500.00",
      tx: "TX: STLR_78SE",
    },
  ];
  return (
    <div className="border border-gray-400 rounded-2xl mt-6 overflow-hidden">
      <div className=" p-6  ">
        <h2 className="text-2xl font-semibold text-gray-800 mb-4">
          Stellar Blockchain Wallet
          <br />
          <span className="text-sm text-gray-500">
            Manage your Stellar wallet for healthcare payments
          </span>
        </h2>
        <div className="flex justify-between items-center border border-gray-300 p-4 shadow rounded ">
          <div>
            <p className="text-base text-gray-600 mb-2 ">Wallet Status</p>
            <p className="text-green-500 bg-green-100 px-2 py-1 rounded-2xl inline-flex items-center">
              <svg
                className="w-4 h-4 mr-1"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M5 13l4 4L19 7"
                />
              </svg>
              Blockchain verified.
            </p>
            <span>Connected to Stellar Network</span>
          </div>
          <div>
            <p className="text-sm text-gray-600">Wallet Balance</p>
            <p className="text-2xl font-bold text-gray-800">$350.00 USD</p>
          </div>
        </div>
      </div>

      {/* Transaction History */}
      <div className=" p-6">
        <div className="border border-gray-300 rounded-2xl p-4">
          <h2 className="text-lg font-semibold text-gray-800 mb-4">
            Transaction History
          </h2>
          {transactions.map((transaction, index) => (
            <div
              key={index}
              className="flex justify-between items-center py-3  last:border-b-0"
            >
              <div>
                <p className="text-gray-800">{transaction.recipient}</p>
                <p className="text-sm text-gray-500">{transaction.date}</p>
              </div>
              <div className="text-right">
               
                <p
                  className={`font-medium ${
                    transaction.amount.startsWith("-")
                      ? "text-red-500"
                      : "text-green-500"
                  }`}
                >
                  {transaction.amount}
                </p>
                <p className="text-sm text-gray-500">{transaction.tx}</p>
              </div>
            </div>
          ))}
        </div>
        <div className="flex justify-between items-center mt-4">
          <button className="bg-[#0096CC] w-[48%] text-white px-4 py-2 rounded hover:bg-[#558ca0]">
            Add Funds
          </button>
          <button className="text-gray-500 flex items-center justify-center border-gray-300 rounded space-x-1 w-[48%] border md:py-2 px-4 ">
            <span className="text-sm">Export Transactions</span>
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2m-4-4l-4 4m0 0l-4-4m4 4V4"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
};

export default StellerPayment;
