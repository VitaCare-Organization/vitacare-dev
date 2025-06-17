import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import CompletedPayments from "./CompletedPayment";
import { Payment, PaymentsByMonth } from "@/types/Payment";

// Mock the ReceiptButton component
jest.mock("@/components/atoms/Button/ReceiptButton", () => {
  return function MockReceiptButton({ children, onClick }: { children: React.ReactNode; onClick: () => void }) {
    return <button onClick={onClick}>{children}</button>;
  };
});

// Mock the arrow icon
jest.mock("@/assets/Arrow.svg", () => "mock-arrow-icon");

describe("CompletedPayments Component", () => {
  const mockPayment: Payment = {
    doctor: "Dr. John Smith",
    specialty: "Cardiology",
    invoiceNumber: "INV-001",
    date: "2024-01-15",
    amount: 150.00
  };

  const mockPaymentsByMonth: PaymentsByMonth = {
    "January 2024": [mockPayment],
    "February 2024": [
      {
        doctor: "Dr. Jane Doe",
        specialty: "Dermatology",
        invoiceNumber: "INV-002",
        date: "2024-02-10",
        amount: 200.00
      }
    ]
  };

  const mockHandleShowReceipt = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it("renders correctly with payment data", () => {
    render(
      <CompletedPayments 
        paymentsByMonth={mockPaymentsByMonth}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    // Check main heading
    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Completed Payments");
    expect(screen.getByText("Your payment history")).toBeInTheDocument();

    // Check month headers
    expect(screen.getByRole("heading", { level: 3, name: "January 2024" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 3, name: "February 2024" })).toBeInTheDocument();
  });

  it("displays payment details correctly", () => {
    render(
      <CompletedPayments 
        paymentsByMonth={mockPaymentsByMonth}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    // Check first payment details
    expect(screen.getByText("Dr. John Smith - Cardiology")).toBeInTheDocument();
    expect(screen.getByText("Invoice #INV-001")).toBeInTheDocument();
    expect(screen.getByText("Paid: 2024-01-15")).toBeInTheDocument();
    expect(screen.getByText("$150.00")).toBeInTheDocument();

    // Check second payment details
    expect(screen.getByText("Dr. Jane Doe - Dermatology")).toBeInTheDocument();
    expect(screen.getByText("Invoice #INV-002")).toBeInTheDocument();
    expect(screen.getByText("Paid: 2024-02-10")).toBeInTheDocument();
    expect(screen.getByText("$200.00")).toBeInTheDocument();
  });

  it("displays checkmark icons for completed payments", () => {
    render(
      <CompletedPayments 
        paymentsByMonth={mockPaymentsByMonth}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    // Check for checkmark SVG elements
    const checkmarks = screen.getAllByRole("img", { hidden: true });
    expect(checkmarks.length).toBeGreaterThan(0);
  });

  it("calls handleShowReceipt when receipt button is clicked", async () => {
    const user = userEvent.setup();
    
    render(
      <CompletedPayments 
        paymentsByMonth={mockPaymentsByMonth}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    const receiptButtons = screen.getAllByRole("button", { name: /receipt/i });
    
    await user.click(receiptButtons[0]);
    
    expect(mockHandleShowReceipt).toHaveBeenCalledTimes(1);
    expect(mockHandleShowReceipt).toHaveBeenCalledWith(mockPayment);
  });

  it("renders with empty payment data", () => {
    render(
      <CompletedPayments 
        paymentsByMonth={{}}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Completed Payments");
    expect(screen.getByText("Your payment history")).toBeInTheDocument();
    
    // Should not have any month headers
    expect(screen.queryByRole("heading", { level: 3 })).not.toBeInTheDocument();
  });

  it("formats payment amounts correctly", () => {
    const paymentWithDecimals: PaymentsByMonth = {
      "March 2024": [{
        doctor: "Dr. Test",
        specialty: "Testing",
        invoiceNumber: "INV-003",
        date: "2024-03-01",
        amount: 99.99
      }]
    };

    render(
      <CompletedPayments 
        paymentsByMonth={paymentWithDecimals}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    expect(screen.getByText("$99.99")).toBeInTheDocument();
  });

  it("has correct styling classes", () => {
    render(
      <CompletedPayments 
        paymentsByMonth={mockPaymentsByMonth}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );

    const mainContainer = screen.getByRole("heading", { level: 2 }).closest("div");
    expect(mainContainer).toHaveClass("bg-white rounded-lg mt-4 shadow-sm border border-gray-200 p-6");
  });

  it("matches snapshot", () => {
    const { container } = render(
      <CompletedPayments 
        paymentsByMonth={mockPaymentsByMonth}
        handleShowReceipt={mockHandleShowReceipt}
      />
    );
    expect(container.firstChild).toMatchSnapshot();
  });
}); 