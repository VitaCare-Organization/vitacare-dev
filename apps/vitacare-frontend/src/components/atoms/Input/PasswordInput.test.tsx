import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom";
import PasswordInput from "./passwordInput";

jest.mock("lucide-react", () => ({
  Eye: () => <div data-testid="eye-icon" />,
  EyeOff: () => <div data-testid="eye-off-icon" />,
}));

describe("PasswordInput", () => {
  it("renders with the provided label", () => {
    render(<PasswordInput label="Password" />);
    expect(screen.getByText("Password")).toBeInTheDocument();
  });

  it("renders with the default password type (hidden)", () => {
    render(<PasswordInput label="Password" />);
    const input = screen.getByLabelText("Password");
    expect(input).toHaveAttribute("type", "password");
  });

  it("toggles password visibility when the button is clicked", () => {
    render(<PasswordInput label="Password" />);
    const input = screen.getByLabelText("Password");
    const toggleButton = screen.getByRole("button", {
      name: /toggle password visibility/i,
    });

    // Initially password should be hidden
    expect(input).toHaveAttribute("type", "password");
    expect(screen.getByTestId("eye-icon")).toBeInTheDocument();

    // Click the toggle button
    fireEvent.click(toggleButton);

    // Password should now be visible
    expect(input).toHaveAttribute("type", "text");
    expect(screen.getByTestId("eye-off-icon")).toBeInTheDocument();

    // Click the toggle button again
    fireEvent.click(toggleButton);

    // Password should be hidden again
    expect(input).toHaveAttribute("type", "password");
    expect(screen.getByTestId("eye-icon")).toBeInTheDocument();
  });

  it("displays an error message when provided", () => {
    const errorMessage = "Password is required";
    render(<PasswordInput label="Password" error={errorMessage} />);
    expect(screen.getByText(errorMessage)).toBeInTheDocument();
    expect(screen.getByText(errorMessage)).toHaveClass("text-red-500");
  });

  it("does not display an error message when not provided", () => {
    render(<PasswordInput label="Password" />);
    const errorElements = screen
      .queryAllByText(/./i)
      .filter(
        (el) =>
          el.tagName.toLowerCase() === "p" &&
          el.classList.contains("text-red-500")
      );
    expect(errorElements.length).toBe(0);
  });

  it("uses the provided id for the input", () => {
    render(<PasswordInput label="Password" id="custom-id" />);
    const input = screen.getByLabelText("Password");
    expect(input).toHaveAttribute("id", "custom-id");
  });

  it("uses a default id if none is provided", () => {
    render(<PasswordInput label="Password" />);
    const input = screen.getByLabelText("Password");
    expect(input).toHaveAttribute("id", "password-input");
  });

  it("passes additional props to the input element", () => {
    render(
      <PasswordInput
        label="Password"
        placeholder="Enter your password"
        required
        data-testid="password-field"
      />
    );

    const input = screen.getByLabelText("Password");
    expect(input).toHaveAttribute("placeholder", "Enter your password");
    expect(input).toHaveAttribute("required");
    expect(input).toHaveAttribute("data-testid", "password-field");
  });
});
