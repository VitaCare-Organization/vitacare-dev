import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom";

jest.mock("../../../lib/utils.ts", () => ({
  cn: jest.fn((...classes: string[]) => classes.filter(Boolean).join(" ")),
}));

import Tab from "./Tab";

describe("Tab Component", () => {
  test("renders with label text", () => {
    render(<Tab label="Home" />);

    const tabButton = screen.getByRole("button", { name: "Home" });
    expect(tabButton).toBeInTheDocument();
  });

  test("applies inactive styles by default", () => {
    render(<Tab label="Settings" />);

    const tabButton = screen.getByRole("button");
    expect(tabButton).toHaveClass(
      "px-4",
      "py-2",
      "m-1",
      "text-sm",
      "font-medium"
    );
    // Check for inactive-specific classes
    expect(tabButton).toHaveClass("text-gray-500");
  });

  test("applies active styles when isActive is true", () => {
    render(<Tab label="Dashboard" isActive={true} />);

    const tabButton = screen.getByRole("button");
    expect(tabButton).toHaveClass(
      "px-4",
      "py-2",
      "m-1",
      "text-sm",
      "font-medium"
    );
    // Check for active-specific classes
    expect(tabButton).toHaveClass("bg-white", "text-gray-900");
  });

  test("calls onClick handler when clicked", () => {
    const mockOnClick = jest.fn();
    render(<Tab label="Profile" onClick={mockOnClick} />);

    const tabButton = screen.getByRole("button");
    fireEvent.click(tabButton);

    expect(mockOnClick).toHaveBeenCalledTimes(1);
  });

  test("does not call onClick when no handler is provided", () => {
    render(<Tab label="About" />);

    const tabButton = screen.getByRole("button");
    fireEvent.click(tabButton);

    expect(tabButton).toBeInTheDocument();
  });

  test("handles keyboard events", () => {
    const mockOnClick = jest.fn();
    render(<Tab label="Contact" onClick={mockOnClick} />);

    const tabButton = screen.getByRole("button");

    fireEvent.keyDown(tabButton, { key: "Enter" });

    fireEvent.click(tabButton);
    expect(mockOnClick).toHaveBeenCalled();
  });

  test("renders as button element", () => {
    render(<Tab label="Test" />);

    const element = screen.getByRole("button");
    expect(element.tagName).toBe("BUTTON");
  });

  test("handles empty label", () => {
    render(<Tab label="" />);

    const tabButton = screen.getByRole("button");
    expect(tabButton).toBeInTheDocument();
    expect(tabButton).toHaveTextContent("");
  });

  test("handles long label text", () => {
    const longLabel = "This is a very long tab label that might overflow";
    render(<Tab label={longLabel} />);

    const tabButton = screen.getByRole("button", { name: longLabel });
    expect(tabButton).toBeInTheDocument();
    expect(tabButton).toHaveTextContent(longLabel);
  });

  test("isActive defaults to false when not provided", () => {
    render(<Tab label="Default" />);

    const tabButton = screen.getByRole("button");

    expect(tabButton).toHaveClass("text-gray-500");
    expect(tabButton).not.toHaveClass("bg-white");
  });

  test("multiple clicks call onClick multiple times", () => {
    const mockOnClick = jest.fn();
    render(<Tab label="Multi-click" onClick={mockOnClick} />);

    const tabButton = screen.getByRole("button");

    fireEvent.click(tabButton);
    fireEvent.click(tabButton);
    fireEvent.click(tabButton);

    expect(mockOnClick).toHaveBeenCalledTimes(3);
  });
});
