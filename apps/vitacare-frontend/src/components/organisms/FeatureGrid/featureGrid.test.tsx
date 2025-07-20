import { render, screen } from "@testing-library/react";
import KeyFeaturesSection from "./featureGrid";

// Mock lucide-react icons
jest.mock("lucide-react", () => ({
  Users: () => <div data-testid="users-icon">Users Icon</div>,
  FileText: () => <div data-testid="filetext-icon">FileText Icon</div>,
  Calendar: () => <div data-testid="calendar-icon">Calendar Icon</div>,
  CreditCard: () => <div data-testid="creditcard-icon">CreditCard Icon</div>,
  Shield: () => <div data-testid="shield-icon">Shield Icon</div>,
  CheckCircle: () => <div data-testid="checkcircle-icon">CheckCircle Icon</div>,
}));

describe("KeyFeaturesSection Component", () => {
  it("renders the main heading and description", () => {
    render(<KeyFeaturesSection />);

    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Key Features");
    expect(screen.getByText(/Our platform provides comprehensive healthcare management/)).toBeInTheDocument();
  });

  it("displays all six feature cards", () => {
    render(<KeyFeaturesSection />);

    // Check all feature titles
    expect(screen.getByText("Role-Based Access")).toBeInTheDocument();
    expect(screen.getByText("Secure Medical Records")).toBeInTheDocument();
    expect(screen.getByText("Appointment System")).toBeInTheDocument();
    expect(screen.getByText("Stellar Payments")).toBeInTheDocument();
    expect(screen.getByText("Enhanced Security")).toBeInTheDocument();
    expect(screen.getByText("Blockchain Verification")).toBeInTheDocument();
  });

  it("displays all feature descriptions", () => {
    render(<KeyFeaturesSection />);

    expect(screen.getByText(/Tailored interfaces for hospitals, doctors, and patients/)).toBeInTheDocument();
    expect(screen.getByText(/Blockchain-verified medical history with secure sharing/)).toBeInTheDocument();
    expect(screen.getByText(/Smart scheduling with availability management/)).toBeInTheDocument();
    expect(screen.getByText(/Verified Stellar blockchain wallet for secure/)).toBeInTheDocument();
    expect(screen.getByText(/Multi-layered using biometrics and encryption/)).toBeInTheDocument();
    expect(screen.getByText(/Stellar indicators for blockchain-verified data/)).toBeInTheDocument();
  });

  it("renders all feature icons", () => {
    render(<KeyFeaturesSection />);

    expect(screen.getByTestId("users-icon")).toBeInTheDocument();
    expect(screen.getByTestId("filetext-icon")).toBeInTheDocument();
    expect(screen.getByTestId("calendar-icon")).toBeInTheDocument();
    expect(screen.getByTestId("creditcard-icon")).toBeInTheDocument();
    expect(screen.getByTestId("shield-icon")).toBeInTheDocument();
    expect(screen.getByTestId("checkcircle-icon")).toBeInTheDocument();
  });

  it("has correct section structure and styling", () => {
    render(<KeyFeaturesSection />);

    const section = screen.getByRole("heading", { level: 2 }).closest("section");
    expect(section).toHaveClass("py-16 px-4 sm:px-6 lg:px-8 bg-gray-50");

    const maxWidthContainer = section?.querySelector(".max-w-7xl");
    expect(maxWidthContainer).toBeInTheDocument();
    expect(maxWidthContainer).toHaveClass("mx-auto");
  });

  it("has correct grid layout for features", () => {
    render(<KeyFeaturesSection />);

    const grid = screen.getByRole("heading", { level: 2 })
      .closest("section")
      ?.querySelector(".grid");
    
    expect(grid).toHaveClass("grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8");
  });

  it("has correct feature card styling", () => {
    render(<KeyFeaturesSection />);

    const featureCards = screen.getAllByText(/Role-Based Access|Secure Medical Records|Appointment System|Stellar Payments|Enhanced Security|Blockchain Verification/)
      .map(title => title.closest(".bg-white"));

    featureCards.forEach(card => {
      expect(card).toHaveClass("bg-white rounded-xl p-8 shadow-sm hover:shadow-md transition-shadow duration-300 border border-gray-100");
    });
  });

  it("has proper heading hierarchy", () => {
    render(<KeyFeaturesSection />);

    // Main heading should be h2
    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Key Features");

    // Feature titles should be h3
    const h3Headings = screen.getAllByRole("heading", { level: 3 });
    expect(h3Headings).toHaveLength(6);
    
    const expectedTitles = [
      "Role-Based Access",
      "Secure Medical Records", 
      "Appointment System",
      "Stellar Payments",
      "Enhanced Security",
      "Blockchain Verification"
    ];
    
    h3Headings.forEach((heading, index) => {
      expect(heading).toHaveTextContent(expectedTitles[index]);
    });
  });

  it("has accessible content structure", () => {
    render(<KeyFeaturesSection />);

    // Check that the section is a proper landmark
    const section = screen.getByRole("heading", { level: 2 }).closest("section");
    expect(section).toBeInTheDocument();

    // All headings should be visible
    const allHeadings = screen.getAllByRole("heading");
    allHeadings.forEach(heading => {
      expect(heading).toBeVisible();
    });

    // All feature descriptions should be visible
    const descriptions = [
      /Tailored interfaces for hospitals/,
      /Blockchain-verified medical history/,
      /Smart scheduling with availability/,
      /Verified Stellar blockchain wallet/,
      /Multi-layered using biometrics/,
      /Stellar indicators for blockchain-verified/
    ];

    descriptions.forEach(descriptionPattern => {
      expect(screen.getByText(descriptionPattern)).toBeVisible();
    });
  });
}); 