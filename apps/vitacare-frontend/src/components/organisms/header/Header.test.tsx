import { render, screen } from "@testing-library/react";
import Navbar from "./Header";

// Mock Next.js Link component
jest.mock("next/link", () => {
  return function MockLink({ children, href }: { children: React.ReactNode; href: string }) {
    return <a href={href}>{children}</a>;
  };
});

// Mock lucide-react icons
jest.mock("lucide-react", () => ({
  Shield: () => <div data-testid="shield-icon">Shield Icon</div>,
}));

describe("Navbar Component", () => {
  it("renders the VitaCare brand correctly", () => {
    render(<Navbar />);

    expect(screen.getByText("VitaCare")).toBeInTheDocument();
    expect(screen.getByTestId("shield-icon")).toBeInTheDocument();
  });

  it("renders all navigation links", () => {
    render(<Navbar />);

    expect(screen.getByRole("link", { name: "Features" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Security" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "About" })).toBeInTheDocument();
  });

  it("renders authentication buttons", () => {
    render(<Navbar />);

    expect(screen.getByRole("link", { name: "Log in" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Register" })).toBeInTheDocument();
  });

  it("has correct href attributes for navigation links", () => {
    render(<Navbar />);

    expect(screen.getByRole("link", { name: "VitaCare" })).toHaveAttribute("href", "/");
    expect(screen.getByRole("link", { name: "Features" })).toHaveAttribute("href", "/features");
    expect(screen.getByRole("link", { name: "Security" })).toHaveAttribute("href", "/security");
    expect(screen.getByRole("link", { name: "About" })).toHaveAttribute("href", "/about");
    expect(screen.getByRole("link", { name: "Log in" })).toHaveAttribute("href", "/login");
    expect(screen.getByRole("link", { name: "Register" })).toHaveAttribute("href", "/register");
  });

  it("has correct header structure and styling", () => {
    render(<Navbar />);

    const header = screen.getByRole("banner");
    expect(header).toHaveClass("w-full border-b border-blue-100 px-8 md:px-20 py-4 md:py-3");
  });

  it("has correct brand section styling", () => {
    render(<Navbar />);

    const brandSection = screen.getByText("VitaCare").closest("div");
    expect(brandSection).toHaveClass("flex items-center");
    
    const brandText = screen.getByRole("link", { name: "VitaCare" });
    expect(brandText).toHaveClass("text-xl font-bold text-gray-800");
  });

  it("has correct navigation styling", () => {
    render(<Navbar />);

    const nav = screen.getByRole("navigation");
    expect(nav).toHaveClass("hidden md:flex items-center space-x-8");
  });

  it("navigation links have correct hover styling", () => {
    render(<Navbar />);

    const featuresLink = screen.getByRole("link", { name: "Features" });
    const securityLink = screen.getByRole("link", { name: "Security" });
    const aboutLink = screen.getByRole("link", { name: "About" });

    expect(featuresLink).toHaveClass("text-gray-600 hover:text-gray-900");
    expect(securityLink).toHaveClass("text-gray-600 hover:text-gray-900");
    expect(aboutLink).toHaveClass("text-gray-600 hover:text-gray-900");
  });

  it("register button has correct styling", () => {
    render(<Navbar />);

    const registerLink = screen.getByRole("link", { name: "Register" });
    expect(registerLink).toHaveClass("bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors");
  });

  it("log in link has correct styling", () => {
    render(<Navbar />);

    const loginLink = screen.getByRole("link", { name: "Log in" });
    expect(loginLink).toHaveClass("text-gray-600 hover:text-gray-900");
  });

  it("has proper semantic structure", () => {
    render(<Navbar />);

    // Should be a header element
    const header = screen.getByRole("banner");
    expect(header.tagName).toBe("HEADER");

    // Should have navigation
    const nav = screen.getByRole("navigation");
    expect(nav.tagName).toBe("NAV");
  });

  it("authentication section has correct styling", () => {
    render(<Navbar />);

    const authSection = screen.getByRole("link", { name: "Log in" }).closest("div");
    expect(authSection).toHaveClass("flex items-center space-x-4");
  });

  it("is accessible with proper landmarks", () => {
    render(<Navbar />);

    // Header should be a landmark
    expect(screen.getByRole("banner")).toBeInTheDocument();
    
    // Navigation should be a landmark
    expect(screen.getByRole("navigation")).toBeInTheDocument();
    
    // All links should be accessible
    const allLinks = screen.getAllByRole("link");
    expect(allLinks.length).toBe(6); // VitaCare, Features, Security, About, Log in, Register
  });
});
