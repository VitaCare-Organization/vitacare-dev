import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import Hero from "./Hero";

// Mock Next.js Image component
jest.mock("next/image", () => {
  return function MockImage({ src, alt }: { src: string; alt: string }) {
    return <img src={src} alt={alt} data-testid="hero-image" />;
  };
});

// Mock the Button component
jest.mock("@/components/atoms/Button/Button", () => ({
  Button: ({ children, className, onClick }: { children: React.ReactNode; className?: string; onClick?: () => void }) => (
    <button className={className} onClick={onClick}>
      {children}
    </button>
  ),
}));

// Mock the hero image
jest.mock("@/assets/Vector.svg", () => "mock-hero-vector.svg");

describe("Hero Component", () => {
  it("renders the main heading correctly", () => {
    render(<Hero />);

    expect(screen.getByRole("heading", { level: 1 })).toHaveTextContent(
      "Secure Healthcare on the Blockchain"
    );
  });

  it("displays the hero description", () => {
    render(<Hero />);

    expect(screen.getByText(/VitaCare connects hospitals, doctors, and patients/)).toBeInTheDocument();
    expect(screen.getByText(/with secure, blockchain-verified medical records/)).toBeInTheDocument();
  });

  it("renders both action buttons", () => {
    render(<Hero />);

    expect(screen.getByRole("button", { name: /get started/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /learn more/i })).toBeInTheDocument();
  });

  it("displays the hero image", () => {
    render(<Hero />);

    const heroImage = screen.getByTestId("hero-image");
    expect(heroImage).toBeInTheDocument();
    expect(heroImage).toHaveAttribute("src", "mock-hero-vector.svg");
    expect(heroImage).toHaveAttribute("alt", "");
  });

  it("has correct main container structure", () => {
    render(<Hero />);

    const mainContainer = screen.getByRole("heading", { level: 1 }).closest("div");
    expect(mainContainer?.parentElement?.parentElement).toHaveClass("min-h-screen flex justify-center items-center");
  });

  it("has proper responsive layout classes", () => {
    render(<Hero />);

    const contentContainer = screen.getByRole("heading", { level: 1 }).closest(".flex");
    expect(contentContainer).toHaveClass("flex flex-col lg:flex-row gap-5 justify-between");
  });

  it("text content has correct styling", () => {
    render(<Hero />);

    const heading = screen.getByRole("heading", { level: 1 });
    expect(heading).toHaveClass("font-bold text-[#020817] text-3xl md:text-4xl md:leading-8 xl:text-6xl lg:leading-16");

    const description = screen.getByText(/VitaCare connects hospitals/);
    expect(description).toHaveClass("text-[#64748B] sm:text-center lg:text-left font-normal text-sm md:text-xl md:leading-7 w-5/6");
  });

  it("Get Started button has correct styling", () => {
    render(<Hero />);

    const getStartedButton = screen.getByRole("button", { name: /get started/i });
    expect(getStartedButton).toHaveClass("text-[#F8FAFC] font-medium bg-[#0096CC] hover:brightness-110 rounded-[6px] py-3 px-8");
  });

  it("Learn More button has correct styling", () => {
    render(<Hero />);

    const learnMoreButton = screen.getByRole("button", { name: /learn more/i });
    expect(learnMoreButton).toHaveClass("border border-[#E2E8F0] bg-white rounded-[6px] py-3 px-8 hover:brightness-90");
  });

  it("buttons container has correct layout", () => {
    render(<Hero />);

    const buttonsContainer = screen.getByRole("button", { name: /get started/i }).closest(".flex");
    expect(buttonsContainer).toHaveClass("flex flex-col min-[400px]:flex-row gap-2");
  });

  it("content section has proper responsive alignment", () => {
    render(<Hero />);

    const textSection = screen.getByRole("heading", { level: 1 }).closest(".flex");
    expect(textSection).toHaveClass("flex flex-col sm:text-center lg:text-left sm:justify-center sm:items-center lg:items-start gap-3 lg:w-1/2");
  });

  it("image section has correct layout", () => {
    render(<Hero />);

    const imageSection = screen.getByTestId("hero-image").closest(".lg\\:w-1\\/2");
    expect(imageSection).toHaveClass("lg:w-1/2 flex justify-end items-end");
  });

  it("is accessible with proper heading structure", () => {
    render(<Hero />);

    // Should have main heading
    expect(screen.getByRole("heading", { level: 1 })).toBeInTheDocument();
    
    // All buttons should be accessible
    const buttons = screen.getAllByRole("button");
    expect(buttons).toHaveLength(2);
    
    // Image should have alt text (even if empty)
    const image = screen.getByTestId("hero-image");
    expect(image).toHaveAttribute("alt");
  });

  it("buttons are clickable", async () => {
    const user = userEvent.setup();
    render(<Hero />);

    const getStartedButton = screen.getByRole("button", { name: /get started/i });
    const learnMoreButton = screen.getByRole("button", { name: /learn more/i });

    // Should be able to click buttons without errors
    await user.click(getStartedButton);
    await user.click(learnMoreButton);
  });

  it("has maximum width constraint", () => {
    render(<Hero />);

    const mainContainer = screen.getByRole("heading", { level: 1 }).closest(".flex");
    expect(mainContainer).toHaveClass("2xl:max-w-[1440px] 2xl:mx-auto");
  });

  it("matches snapshot", () => {
    const { container } = render(<Hero />);
    expect(container.firstChild).toMatchSnapshot();
  });
}); 