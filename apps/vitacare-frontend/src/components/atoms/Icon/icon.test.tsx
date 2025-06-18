import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import Icon from "./Icon";

describe("Icon Component", () => {
  test("renders with children content", () => {
    render(<Icon>📍</Icon>);

    const iconElement = screen.getByText("📍");
    expect(iconElement).toBeInTheDocument();
  });

  test("applies correct CSS classes", () => {
    render(<Icon>🔍</Icon>);

    const iconContainer = screen.getByText("🔍");
    expect(iconContainer).toHaveClass("text-blue-500", "text-3xl");
  });

  test("renders with text content as children", () => {
    render(<Icon>search</Icon>);

    const iconElement = screen.getByText("search");
    expect(iconElement).toBeInTheDocument();
  });

  test("renders as a div element", () => {
    render(<Icon>test</Icon>);

    const iconContainer = screen.getByText("test");
    expect(iconContainer.tagName).toBe("DIV");
  });

  test("handles empty children", () => {
    render(<Icon>{""}</Icon>);

    const container = document.querySelector(".text-blue-500.text-3xl");
    expect(container).toBeInTheDocument();
    expect(container).toBeEmptyDOMElement();
  });
});
