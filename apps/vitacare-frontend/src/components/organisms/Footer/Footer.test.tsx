import { render, screen } from "@testing-library/react";
import Footer from "./Footer";

// Mock lucide-react icons
jest.mock("lucide-react", () => ({
  Shield: () => <div data-testid="shield-icon">Shield Icon</div>,
}));

describe("Footer Component", () => {
  it("renders the VitaCare brand correctly", () => {
    render(<Footer />);

    expect(screen.getByText("VitaCare")).toBeInTheDocument();
    expect(screen.getByTestId("shield-icon")).toBeInTheDocument();
  });

  it("displays the copyright notice with current year", () => {
    render(<Footer />);

    const currentYear = new Date().getFullYear();
    const copyrightText = `© ${currentYear} VitaCare. All rights reserved. Powered by Stellar blockchain.`;
    
    expect(screen.getByText(copyrightText)).toBeInTheDocument();
  });

  it("renders all footer links", () => {
    render(<Footer />);

    expect(screen.getByText("Terms")).toBeInTheDocument();
    expect(screen.getByText("Privacy")).toBeInTheDocument();
    expect(screen.getByText("Contact")).toBeInTheDocument();
  });

  it("has correct footer structure and styling", () => {
    render(<Footer />);

    const footer = screen.getByRole("contentinfo");
    expect(footer).toHaveClass("flex flex-col md:flex-row justify-between items-center px-8 md:px-20 py-5 md:py-7 border-t border-[#E2E8F0] text-center md:text-left");
  });

  it("brand section has correct styling", () => {
    render(<Footer />);

    const brandSection = screen.getByText("VitaCare").closest("div");
    expect(brandSection).toHaveClass("flex items-center space-x-2");
    
    const brandText = screen.getByText("VitaCare");
    expect(brandText).toHaveClass("font-bold text-xl text-[#020817]");
  });

  it("copyright text has correct styling", () => {
    render(<Footer />);

    const currentYear = new Date().getFullYear();
    const copyrightElement = screen.getByText(`© ${currentYear} VitaCare. All rights reserved. Powered by Stellar blockchain.`);
    
    expect(copyrightElement).toHaveClass("text-[#64748B] mt-2 md:mt-0");
  });

  it("footer links have correct styling and cursor pointer", () => {
    render(<Footer />);

    const linksList = screen.getByText("Terms").closest("ul");
    expect(linksList).toHaveClass("flex space-x-4 text-[#64748B] mt-2 md:mt-0 *:cursor-pointer");
  });

  it("links are clickable elements", () => {
    render(<Footer />);

    const termsLink = screen.getByText("Terms");
    const privacyLink = screen.getByText("Privacy");
    const contactLink = screen.getByText("Contact");

    // Check they are list items (clickable)
    expect(termsLink.tagName).toBe("LI");
    expect(privacyLink.tagName).toBe("LI");
    expect(contactLink.tagName).toBe("LI");
  });

  it("has proper semantic structure", () => {
    render(<Footer />);

    // Should be a footer element
    const footer = screen.getByRole("contentinfo");
    expect(footer.tagName).toBe("FOOTER");
  });

  it("displays current year dynamically", () => {
    render(<Footer />);

    const currentYear = new Date().getFullYear();
    expect(screen.getByText(new RegExp(`© ${currentYear}`))).toBeInTheDocument();
  });
});
