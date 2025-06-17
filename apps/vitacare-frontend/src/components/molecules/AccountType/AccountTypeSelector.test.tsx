import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom";
import AccountTypeSelector from "./AccountTypeSelector";

describe("AccountTypeSelector", () => {
  const mockSetValue = jest.fn();

  beforeEach(() => {
    mockSetValue.mockClear();
  });

  it("renders all account types", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);
    expect(screen.getByText("Patient")).toBeInTheDocument();
    expect(screen.getByText("Doctor")).toBeInTheDocument();
    expect(screen.getByText("Hospital")).toBeInTheDocument();
  });

  it("renders with no selection by default when no selectedType is provided", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);
    const radioButtons = screen.getAllByRole("radio");
    radioButtons.forEach((radio) => {
      expect(radio).not.toBeChecked();
    });
  });

  it("renders with the provided selectedType selected", () => {
    render(
      <AccountTypeSelector setValue={mockSetValue} selectedType="Doctor" />
    );
    const radioButtons = document.querySelectorAll('input[type="radio"]');
    const doctorRadio = Array.from(radioButtons).find(
      (radio) => radio.getAttribute("value") === "Doctor"
    );
    expect(doctorRadio).toHaveAttribute("checked", "");
  });

  it("calls setValue when an account type is selected", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);
    fireEvent.click(screen.getByText("Doctor"));
    expect(mockSetValue).toHaveBeenCalledWith("accountType", "Doctor", {
      shouldValidate: true,
    });
  });

  it("updates the visual selection when an account type is clicked", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);

    fireEvent.click(screen.getByText("Doctor"));

    const doctorRadio = screen.getByDisplayValue("Doctor");
    expect(doctorRadio).toBeChecked();

    fireEvent.click(screen.getByText("Hospital"));

    const hospitalRadio = screen.getByDisplayValue("Hospital");
    expect(hospitalRadio).toBeChecked();
    expect(doctorRadio).not.toBeChecked();
  });
});
