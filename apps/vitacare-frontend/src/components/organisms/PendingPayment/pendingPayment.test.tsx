import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import PendingPayments from "./pendingPayment";

type PendingPayment = {
  doctor: string;
  specialty: string;
  invoice: string;
  dueDate: string;
  amount: number;
};

describe("PendingPayments Component", () => {
  const mockPendingPayments: PendingPayment[] = [
    {
      doctor: "Dr. John Smith",
      specialty: "Cardiology",
      invoice: "INV-001",
      dueDate: "2024-12-31",
      amount: 150.00
    },
    {
      doctor: "Dr. Jane Doe",
      specialty: "Dermatology",
      invoice: "INV-002",
      dueDate: "2024-12-15",
      amount: 200.50
    }
  ];

  beforeEach(() => {
    // Clear console.log mock if needed
    jest.clearAllMocks();
  });

  it("renders the main heading and description", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Pending Payments");
    expect(screen.getByText("Payments that require your attention")).toBeInTheDocument();
  });

  it("displays all pending payments", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    expect(screen.getByText("Dr. John Smith - Cardiology")).toBeInTheDocument();
    expect(screen.getByText("Dr. Jane Doe - Dermatology")).toBeInTheDocument();
  });

  it("shows invoice numbers correctly", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    expect(screen.getByText("Invoice #INV-001")).toBeInTheDocument();
    expect(screen.getByText("Invoice #INV-002")).toBeInTheDocument();
  });

  it("displays due dates correctly", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    expect(screen.getByText("Due: 2024-12-31")).toBeInTheDocument();
    expect(screen.getByText("Due: 2024-12-15")).toBeInTheDocument();
  });

  it("formats payment amounts correctly", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    expect(screen.getByText("$150.00")).toBeInTheDocument();
    expect(screen.getByText("$200.50")).toBeInTheDocument();
  });

  it("renders pay now buttons for each payment", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const payButtons = screen.getAllByTestId("pay-now-button");
    expect(payButtons).toHaveLength(2);
  });

  it("calls handlePayNow when pay button is clicked", async () => {
    const consoleSpy = jest.spyOn(console, 'log').mockImplementation();
    const user = userEvent.setup();
    
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const firstPayButton = screen.getAllByTestId("pay-now-button")[0];
    await user.click(firstPayButton);

    expect(consoleSpy).toHaveBeenCalledWith("Processing payment for:", mockPendingPayments[0]);
    
    consoleSpy.mockRestore();
  });

  it("renders with empty payments array", () => {
    render(<PendingPayments pendingPayments={[]} />);

    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Pending Payments");
    expect(screen.getByText("Payments that require your attention")).toBeInTheDocument();
    
    // Should not have any payment items
    expect(screen.queryByText(/Dr\./)).not.toBeInTheDocument();
    expect(screen.queryByTestId("pay-now-button")).not.toBeInTheDocument();
  });

  it("has correct main container styling", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const mainContainer = screen.getByRole("heading", { level: 2 }).closest(".bg-white");
    expect(mainContainer).toHaveClass("bg-white rounded-lg mt-4 shadow-sm border border-gray-200 p-6");
  });

  it("has correct payment item styling", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const paymentItems = screen.getAllByText(/Dr\./).map(item => 
      item.closest(".p-4")
    );

    paymentItems.forEach(item => {
      expect(item).toHaveClass("p-4 border-b last:border-b-0");
    });
  });

  it("displays payment information in correct layout", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const firstPaymentContainer = screen.getByText("Dr. John Smith - Cardiology").closest(".flex");
    expect(firstPaymentContainer).toHaveClass("flex leading-7 flex-col md:flex-row md:items-center md:justify-between");
  });

  it("amount and button section has correct styling", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const amountSection = screen.getByText("$150.00").closest(".flex");
    expect(amountSection).toHaveClass("mt-4 md:mt-0 flex items-center space-x-4");
  });

  it("handles single payment correctly", () => {
    const singlePayment = [mockPendingPayments[0]];
    render(<PendingPayments pendingPayments={singlePayment} />);

    expect(screen.getByText("Dr. John Smith - Cardiology")).toBeInTheDocument();
    expect(screen.getAllByTestId("pay-now-button")).toHaveLength(1);
  });

  it("payment amounts are displayed as semibold", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const amountElements = screen.getAllByText(/\$\d+\.\d{2}/);
    amountElements.forEach(element => {
      expect(element).toHaveClass("text-lg font-semibold");
    });
  });

  it("doctor information has correct styling", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const doctorNames = screen.getAllByText(/Dr\. .+ - .+/);
    doctorNames.forEach(name => {
      expect(name).toHaveClass("font-medium text-gray-800");
    });
  });

  it("invoice and due date text has correct styling", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    const invoiceTexts = screen.getAllByText(/Invoice #/);
    const dueDateTexts = screen.getAllByText(/Due:/);

    [...invoiceTexts, ...dueDateTexts].forEach(text => {
      expect(text).toHaveClass("text-sm text-gray-500");
    });
  });

  it("is accessible with proper heading structure", () => {
    render(<PendingPayments pendingPayments={mockPendingPayments} />);

    // Should have main heading
    expect(screen.getByRole("heading", { level: 2 })).toBeInTheDocument();
    
    // All buttons should be accessible
    const buttons = screen.getAllByRole("button");
    expect(buttons.length).toBe(mockPendingPayments.length);
  });
}); 