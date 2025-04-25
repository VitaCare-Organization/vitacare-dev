import React from "react";
import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import Input from "./Input";

describe("Input", () => {
  it("renders with the provided label", () => {
    render(<Input label="Email Address" />);
    expect(screen.getByText("Email Address")).toBeInTheDocument();
  });

  it("forwards ref to the input element", () => {
    const ref = React.createRef<HTMLInputElement>();
    render(<Input label="Email" ref={ref} />);
    expect(ref.current).not.toBeNull();
    expect(ref.current?.tagName).toBe("INPUT");
  });

  it("uses the provided id for the input", () => {
    render(<Input label="Email" id="custom-email-id" />);
    const input = screen.getByLabelText("Email");
    expect(input).toHaveAttribute("id", "custom-email-id");
  });

  it("generates an id based on the label if none is provided", () => {
    render(<Input label="Email Address" />);
    const input = screen.getByLabelText("Email Address");
    expect(input).toHaveAttribute("id", "email-address");
  });

  it("displays an error message when provided", () => {
    const errorMessage = "Email is required";
    render(<Input label="Email" error={errorMessage} />);
    expect(screen.getByText(errorMessage)).toBeInTheDocument();
    expect(screen.getByText(errorMessage)).toHaveClass("text-red-500");
  });

  it("does not display an error message when not provided", () => {
    render(<Input label="Email" />);
    const errorElements = screen
      .queryAllByText(/./i)
      .filter(
        (el) =>
          el.tagName.toLowerCase() === "p" &&
          el.classList.contains("text-red-500")
      );
    expect(errorElements.length).toBe(0);
  });

  it("passes additional props to the input element", () => {
    render(
      <Input
        label="Email"
        placeholder="Enter your email"
        type="email"
        required
        data-testid="email-field"
      />
    );

    const input = screen.getByLabelText("Email");
    expect(input).toHaveAttribute("placeholder", "Enter your email");
    expect(input).toHaveAttribute("type", "email");
    expect(input).toHaveAttribute("required");
    expect(input).toHaveAttribute("data-testid", "email-field");
  });

  it("applies the correct CSS classes to the input", () => {
    render(<Input label="Email" />);
    const input = screen.getByLabelText("Email");

    expect(input).toHaveClass("mt-1");
    expect(input).toHaveClass("p-2");
    expect(input).toHaveClass("border");
    expect(input).toHaveClass("border-[#E2E8F0]");
    expect(input).toHaveClass("rounded-[6px]");
    expect(input).toHaveClass("text-[#020817]");
    expect(input).toHaveClass("focus:ring-2");
    expect(input).toHaveClass("focus:ring-slate-300");
  });

  it("handles special characters in label when generating id", () => {
    render(<Input label="User's Email (Primary)" />);
    const input = screen.getByLabelText("User's Email (Primary)");
    expect(input).toHaveAttribute("id", "user's-email-(primary)");
  });
});
