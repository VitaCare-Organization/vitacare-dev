import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { Heading, Description } from "./Text";

describe("Text Components", () => {
  describe("Heading Component", () => {
    test("renders heading text content", () => {
      render(<Heading>Welcome to Dashboard</Heading>);

      const heading = screen.getByRole("heading", { level: 3 });
      expect(heading).toBeInTheDocument();
      expect(heading).toHaveTextContent("Welcome to Dashboard");
    });

    test("applies correct CSS classes", () => {
      render(<Heading>Test Heading</Heading>);

      const heading = screen.getByRole("heading");
      expect(heading).toHaveClass("text-lg", "font-semibold", "text-gray-900");
    });

    test("renders as h3 element", () => {
      render(<Heading>Page Title</Heading>);

      const heading = screen.getByRole("heading");
      expect(heading.tagName).toBe("H3");
    });

    test("handles empty string", () => {
      render(<Heading>{""}</Heading>);

      const heading = screen.getByRole("heading");
      expect(heading).toBeInTheDocument();
      expect(heading).toHaveTextContent("");
    });

    test("handles long text content", () => {
      const longText =
        "This is a very long heading that might span multiple lines and test how the component handles extensive text content";
      render(<Heading>{longText}</Heading>);

      const heading = screen.getByRole("heading");
      expect(heading).toHaveTextContent(longText);
    });

    test("handles special characters and numbers", () => {
      const specialText = "User Profile 123 Settings";
      render(<Heading>{specialText}</Heading>);

      const heading = screen.getByRole("heading");
      expect(heading).toHaveTextContent(specialText);
    });

    test("handles text with spaces", () => {
      render(<Heading> Trimmed Text </Heading>);

      const heading = screen.getByRole("heading");
      expect(heading).toHaveTextContent("Trimmed Text");
    });
  });

  describe("Description Component", () => {
    test("renders description text content", () => {
      render(<Description>This is a helpful description</Description>);

      const description = screen.getByText("This is a helpful description");
      expect(description).toBeInTheDocument();
    });

    test("applies correct CSS classes", () => {
      render(<Description>Test Description</Description>);

      const description = screen.getByText("Test Description");
      expect(description).toHaveClass("text-gray-600");
    });

    test("renders as paragraph element", () => {
      render(<Description>Sample text</Description>);

      const description = screen.getByText("Sample text");
      expect(description.tagName).toBe("P");
    });

    test("handles empty string", () => {
      render(<Description>{""}</Description>);
      const description = document.querySelector(".text-gray-600");
      expect(description).toBeInTheDocument();
      expect(description).toHaveTextContent("");
    });

    test("handles long descriptive text", () => {
      const longDescription =
        "This is a comprehensive description that provides detailed information about the current section or feature. It may contain multiple sentences and explain various aspects of the functionality to help users understand what they are looking at.";
      render(<Description>{longDescription}</Description>);

      const description = screen.getByText(longDescription);
      expect(description).toHaveTextContent(longDescription);
    });

    test("handles text with line breaks and special characters", () => {
      const complexText =
        "Description with numbers: 123, symbols and punctuation";
      render(<Description>{complexText}</Description>);

      const description = screen.getByText(complexText);
      expect(description).toHaveTextContent(complexText);
    });

    test("handles whitespace correctly", () => {
      render(<Description> Spaced text </Description>);

      const description = document.querySelector(".text-gray-600");
      expect(description).toBeInTheDocument();
      expect(description).toHaveTextContent("Spaced text");
    });
  });

  describe("Text Components Integration", () => {
    test("both components can be rendered together", () => {
      render(
        <div>
          <Heading>Main Title</Heading>
          <Description>Supporting description text</Description>
        </div>
      );

      const heading = screen.getByRole("heading");
      const description = screen.getByText("Supporting description text");

      expect(heading).toBeInTheDocument();
      expect(description).toBeInTheDocument();
      expect(heading).toHaveTextContent("Main Title");
    });

    test("components maintain different styling", () => {
      render(
        <div>
          <Heading>Title</Heading>
          <Description>Description</Description>
        </div>
      );

      const heading = screen.getByRole("heading");
      const description = screen.getByText("Description");

      expect(heading).toHaveClass("text-lg", "font-semibold", "text-gray-900");
      expect(description).toHaveClass("text-gray-600");
      expect(description).not.toHaveClass("text-lg", "font-semibold");
    });
  });
});
