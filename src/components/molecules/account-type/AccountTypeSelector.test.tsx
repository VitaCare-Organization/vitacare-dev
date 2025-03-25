import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom";
import AccountTypeSelector from "./AccountTypeSelector";

describe("AccountTypeSelector", () => {
  // Mock the setValue function from react-hook-form
  const mockSetValue = jest.fn();

  beforeEach(() => {
    // Clear mock calls between tests
    mockSetValue.mockClear();
  });

  it("renders all account types", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);

    // Check if all account types are rendered
    expect(screen.getByText("Patient")).toBeInTheDocument();
    expect(screen.getByText("Doctor")).toBeInTheDocument();
    expect(screen.getByText("Hospital")).toBeInTheDocument();
  });

  it("renders with no selection by default when no selectedType is provided", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);

    // Check that no radio button is selected
    const radioButtons = screen.getAllByRole("radio");
    radioButtons.forEach((radio) => {
      expect(radio).not.toBeChecked();
    });
  });

  it("renders with the provided selectedType selected", () => {
    render(
      <AccountTypeSelector setValue={mockSetValue} selectedType="Doctor" />
    );

    // Get all radio inputs
    const radioButtons = document.querySelectorAll('input[type="radio"]');

    // Find the Doctor radio button and check if it's selected
    const doctorRadio = Array.from(radioButtons).find(
      (radio) => radio.getAttribute("value") === "Doctor"
    );

    expect(doctorRadio).toHaveAttribute("checked", "");
  });

  it("calls setValue when an account type is selected", () => {
    render(<AccountTypeSelector setValue={mockSetValue} />);

    // Find the Doctor label and click it
    fireEvent.click(screen.getByText("Doctor"));

    // Check if setValue was called with the correct arguments
    expect(mockSetValue).toHaveBeenCalledWith("accountType", "Doctor", {
      shouldValidate: true,
    });
  });

  it("updates the visual selection when an account type is clicked", () => {
    const { container } = render(
      <AccountTypeSelector setValue={mockSetValue} />
    );

    // Click on the Doctor option
    fireEvent.click(screen.getByText("Doctor"));

    // Check if the Doctor option has the selected styling
    // We're looking for the inner div that appears when selected
    const doctorLabel = screen.getByText("Doctor").closest("label");
    const innerDiv = doctorLabel?.querySelector(".bg-[#0096CC]");
    expect(innerDiv).toBeInTheDocument();

    // Click on the Hospital option
    fireEvent.click(screen.getByText("Hospital"));

    // Check if the Hospital option now has the selected styling
    const hospitalLabel = screen.getByText("Hospital").closest("label");
    const hospitalInnerDiv = hospitalLabel?.querySelector(".bg-[#0096CC]");
    expect(hospitalInnerDiv).toBeInTheDocument();

    // And Doctor should no longer be selected
    const doctorInnerDivAfterChange =
      doctorLabel?.querySelector(".bg-[#0096CC]");
    expect(doctorInnerDivAfterChange).not.toBeInTheDocument();
  });
});
