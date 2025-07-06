import { render, screen } from "@testing-library/react";
import AccountHeader from "./AccountHeader";

describe("AccountHeader Component", () => {
  it("renders correctly with all elements", () => {
    render(<AccountHeader />);
    
    // Check for the main heading
    expect(screen.getByRole("heading", { level: 1 })).toHaveTextContent("Create an account");
    
    // Check for the description text
    expect(screen.getByText("Register to access the VitaCare platform")).toBeInTheDocument();
  });

  it("displays the user icon correctly", () => {
    render(<AccountHeader />);
    
    // Check for the icon container
    const iconContainer = screen.getByTestId("account-header").querySelector('.w-10.h-10');
    expect(iconContainer).toHaveClass("w-10 h-10 rounded-full bg-blue-50 flex items-center justify-center");
    
    // Check for SVG presence
    const svg = iconContainer?.querySelector("svg");
    expect(svg).toBeInTheDocument();
    expect(svg).toHaveAttribute("width", "24");
    expect(svg).toHaveAttribute("height", "24");
  });

  it("has correct styling classes", () => {
    render(<AccountHeader />);
    
    const container = screen.getByRole("heading", { level: 1 }).closest("div");
    expect(container).toHaveClass("text-center space-y-2 mb-6");
    
    const heading = screen.getByRole("heading", { level: 1 });
    expect(heading).toHaveClass("text-2xl font-bold text-gray-900");
    
    const description = screen.getByText("Register to access the VitaCare platform");
    expect(description).toHaveClass("text-sm text-gray-500");
  });

  it("renders with proper accessibility", () => {
    render(<AccountHeader />);
    
    // Check that the heading is properly structured
    const heading = screen.getByRole("heading", { level: 1 });
    expect(heading).toBeInTheDocument();
    
    // Check that the description is visible to screen readers
    const description = screen.getByText("Register to access the VitaCare platform");
    expect(description).toBeVisible();
  });
}); 