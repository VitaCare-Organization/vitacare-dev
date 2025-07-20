import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom";
import FormField from "./formField";

describe("FormField", () => {
  const mockLabel = "Test Label";
  const mockId = "test-input";
  const mockOnChange = jest.fn();

  beforeEach(() => {
    mockOnChange.mockClear();
  });

  it("renders with required props", () => {
    render(<FormField label={mockLabel} id={mockId} />);

    expect(screen.getByLabelText(mockLabel)).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toHaveAttribute("id", mockId);
  });

  it("renders with all optional props", () => {
    const mockIcon = <svg data-testid="mock-icon" />;
    render(
      <FormField
        label={mockLabel}
        id={mockId}
        type="email"
        placeholder="Enter email"
        required={true}
        icon={mockIcon}
        value="test@example.com"
        onChange={mockOnChange}
      />
    );

    const input = screen.getByRole("textbox");
    expect(input).toHaveAttribute("type", "email");
    expect(input).toHaveAttribute("placeholder", "Enter email");
    expect(input).toBeRequired();
    expect(input).toHaveValue("test@example.com");
    expect(screen.getByTestId("mock-icon")).toBeInTheDocument();
  });

  it("handles onChange events", () => {
    render(
      <FormField
        label={mockLabel}
        id={mockId}
        value=""
        onChange={mockOnChange}
      />
    );

    const input = screen.getByRole("textbox");
    fireEvent.change(input, { target: { value: "new value" } });
    expect(mockOnChange).toHaveBeenCalledTimes(1);
  });

  it("renders without icon when not provided", () => {
    render(<FormField label={mockLabel} id={mockId} />);
    expect(screen.queryByTestId("mock-icon")).not.toBeInTheDocument();
  });

  it("uses default values when optional props are not provided", () => {
    render(<FormField label={mockLabel} id={mockId} />);

    const input = screen.getByRole("textbox");
    expect(input).toHaveAttribute("type", "text");
    expect(input).toHaveAttribute("placeholder", "");
    expect(input).not.toBeRequired();
    expect(input).not.toHaveValue();
  });
});
