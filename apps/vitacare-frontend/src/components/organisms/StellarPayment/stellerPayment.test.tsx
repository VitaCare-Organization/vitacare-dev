import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import StellerPayment from "./stellerPayment";

describe("StellerPayment Component", () => {
  it("displays wallet status as blockchain verified", () => {
    render(<StellerPayment />);

    expect(screen.getByText("Wallet Status")).toBeInTheDocument();
    expect(screen.getByText("Blockchain verified.")).toBeInTheDocument();
    expect(screen.getByText("Connected to Stellar Network")).toBeInTheDocument();
  });

  it("shows the wallet balance", () => {
    render(<StellerPayment />);

    expect(screen.getByText("Wallet Balance")).toBeInTheDocument();
    expect(screen.getByText("$350.00 USD")).toBeInTheDocument();
  });

  it("renders transaction history section", () => {
    render(<StellerPayment />);

    expect(screen.getByRole("heading", { name: /transaction history/i })).toBeInTheDocument();
  });

  it("displays all transaction entries", () => {
    render(<StellerPayment />);

    expect(screen.getByText("City General Hospital")).toBeInTheDocument();
    expect(screen.getByText("Bright Smile Dental")).toBeInTheDocument();
    expect(screen.getByText("From Bank Account")).toBeInTheDocument();
  });

  it("shows transaction dates", () => {
    render(<StellerPayment />);

    expect(screen.getByText("February 15, 2025")).toBeInTheDocument();
    expect(screen.getByText("February 1, 2025")).toBeInTheDocument();
    expect(screen.getByText("January 25, 2025")).toBeInTheDocument();
  });

  it("displays transaction amounts with correct colors", () => {
    render(<StellerPayment />);

    const negativeAmounts = screen.getAllByText(/^-\$/);
    const positiveAmount = screen.getByText("+$500.00");

    negativeAmounts.forEach(amount => {
      expect(amount).toHaveClass("text-red-500");
    });

    expect(positiveAmount).toHaveClass("text-green-500");
  });

  it("shows transaction IDs", () => {
    render(<StellerPayment />);

    expect(screen.getByText("TX: STLR_AFRD")).toBeInTheDocument();
    expect(screen.getByText("TX: STLR_BASIC")).toBeInTheDocument();
    expect(screen.getByText("TX: STLR_78SE")).toBeInTheDocument();
  });

  it("renders action buttons", () => {
    render(<StellerPayment />);

    expect(screen.getByRole("button", { name: /add funds/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /export transactions/i })).toBeInTheDocument();
  });

  it("Add Funds button has correct styling", () => {
    render(<StellerPayment />);

    const addFundsButton = screen.getByRole("button", { name: /add funds/i });
    expect(addFundsButton).toHaveClass("bg-[#0096CC] w-[48%] text-white px-4 py-2 rounded hover:bg-[#558ca0]");
  });

  it("Export Transactions button has correct styling", () => {
    render(<StellerPayment />);

    const exportButton = screen.getByRole("button", { name: /export transactions/i });
    expect(exportButton).toHaveClass("text-gray-500 flex items-center justify-center border-gray-300 rounded space-x-1 w-[48%] border md:py-2 px-4");
  });

  it("buttons are clickable", async () => {
    const user = userEvent.setup();
    render(<StellerPayment />);

    const addFundsButton = screen.getByRole("button", { name: /add funds/i });
    const exportButton = screen.getByRole("button", { name: /export transactions/i });

    await user.click(addFundsButton);
    await user.click(exportButton);
  });
}); 