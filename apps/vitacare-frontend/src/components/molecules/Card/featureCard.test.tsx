import React from "react";
import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import FeatureCard from "./featureCard";

describe("FeatureCard", () => {
  const mockIcon = <svg data-testid="mock-icon" />;
  const mockTitle = "Test Feature";
  const mockDescription = "This is a test description";

  it("renders with provided props", () => {
    render(
      <FeatureCard
        icon={mockIcon}
        title={mockTitle}
        description={mockDescription}
      />
    );

    expect(screen.getByTestId("mock-icon")).toBeInTheDocument();
    expect(screen.getByText(mockTitle)).toBeInTheDocument();
    expect(screen.getByText(mockDescription)).toBeInTheDocument();
  });

  it("renders icon within Icon component", () => {
    render(
      <FeatureCard
        icon={mockIcon}
        title={mockTitle}
        description={mockDescription}
      />
    );

    const iconContainer = screen.getByTestId("mock-icon").parentElement;
    expect(iconContainer).toHaveClass("text-blue-500 text-3xl");
  });

  it("renders title as heading element", () => {
    render(
      <FeatureCard
        icon={mockIcon}
        title={mockTitle}
        description={mockDescription}
      />
    );

    const heading = screen.getByRole("heading");
    expect(heading).toHaveTextContent(mockTitle);
    expect(heading.tagName).toBe("H3");
  });

  it("renders description as paragraph element", () => {
    render(
      <FeatureCard
        icon={mockIcon}
        title={mockTitle}
        description={mockDescription}
      />
    );

    const description = screen.getByText(mockDescription);
    expect(description.tagName).toBe("P");
  });

  it("handles empty title gracefully", () => {
    render(
      <FeatureCard
        icon={mockIcon}
        title=""
        description={mockDescription}
      />
    );

    expect(screen.getByTestId("mock-icon")).toBeInTheDocument();
    expect(screen.getByText(mockDescription)).toBeInTheDocument();
    
    const heading = screen.getByRole("heading");
    expect(heading).toBeEmptyDOMElement();
  });

  it("handles empty description gracefully", () => {
    render(
      <FeatureCard
        icon={mockIcon}
        title={mockTitle}
        description=""
      />
    );

    expect(screen.getByTestId("mock-icon")).toBeInTheDocument();
    expect(screen.getByText(mockTitle)).toBeInTheDocument();
    
    // Find the paragraph element that should contain the description
    const paragraphs = screen.getAllByRole("paragraph");
    const descriptionParagraph = paragraphs.find(p => p.textContent === "");
    expect(descriptionParagraph).toBeEmptyDOMElement();
  });

  it("handles both empty title and description gracefully", () => {
    render(<FeatureCard icon={mockIcon} title="" description="" />);

    expect(screen.getByTestId("mock-icon")).toBeInTheDocument();
    
    const heading = screen.getByRole("heading");
    expect(heading).toBeEmptyDOMElement();
    
    const paragraphs = screen.getAllByRole("paragraph");
    const descriptionParagraph = paragraphs.find(p => p.textContent === "");
    expect(descriptionParagraph).toBeEmptyDOMElement();
  });
});
