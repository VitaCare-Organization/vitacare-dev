import { render, screen } from "@testing-library/react";
import Register from "./register";

describe("Register Component", () => {
  it("renders all child components", () => {
    render(<Register />);

    expect(screen.getByTestId("back-button")).toBeInTheDocument();
    expect(screen.getByTestId("account-header")).toBeInTheDocument();
    expect(screen.getByTestId("registration-form")).toBeInTheDocument();
  });

  it("has correct main container structure", () => {
    render(<Register />);

    const mainContainer = screen.getByTestId("back-button").closest(".min-h-screen");
    expect(mainContainer).toHaveClass("min-h-screen flex items-center justify-center p-4");
  });

  it("has correct card container styling", () => {
    render(<Register />);

    const cardContainer = screen.getByTestId("back-button").closest(".bg-white");
    expect(cardContainer).toHaveClass("bg-white rounded-lg shadow-lg w-full max-w-4xl overflow-hidden");
  });

  it("has correct content padding", () => {
    render(<Register />);

    const contentContainer = screen.getByTestId("back-button").closest(".p-6");
    expect(contentContainer).toHaveClass("p-6 md:p-8");
  });

  it("back button section has correct margin", () => {
    render(<Register />);

    const backButtonSection = screen.getByTestId("back-button").closest(".mb-8");
    expect(backButtonSection).toHaveClass("mb-8");
  });

  it("form section has correct max width and centering", () => {
    render(<Register />);

    const formSection = screen.getByTestId("account-header").closest(".max-w-md");
    expect(formSection).toHaveClass("max-w-md mx-auto");
  });

  it("renders in correct order", () => {
    const { container } = render(<Register />);

    const elements = container.querySelectorAll('[data-testid]');
    const elementIds = Array.from(elements).map(el => el.getAttribute('data-testid'));

    expect(elementIds).toEqual([
      'back-button',
      'account-header', 
      'registration-form'
    ]);
  });

  it("is properly structured for accessibility", () => {
    render(<Register />);

    // All components should be present and accessible
    expect(screen.getByTestId("back-button")).toBeVisible();
    expect(screen.getByTestId("account-header")).toBeVisible();
    expect(screen.getByTestId("registration-form")).toBeVisible();
  });

  it("has responsive design classes", () => {
    render(<Register />);

    // Check for responsive padding classes
    const contentContainer = screen.getByTestId("back-button").closest(".p-6");
    expect(contentContainer).toHaveClass("p-6 md:p-8");
  });

  it("maintains proper component hierarchy", () => {
    render(<Register />);

    const mainDiv = screen.getByTestId("back-button").closest("div");
    const containerDiv = mainDiv?.querySelector(".min-h-screen");
    expect(containerDiv).toBeInTheDocument();

    const cardDiv = containerDiv?.querySelector(".bg-white");
    expect(cardDiv).toBeInTheDocument();

    const contentDiv = cardDiv?.querySelector(".p-6");
    expect(contentDiv).toBeInTheDocument();
  });
}); 