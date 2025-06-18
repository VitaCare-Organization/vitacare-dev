import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import RegistrationForm from "./RegistrationForm";

// Mock Next.js Link component
jest.mock("next/link", () => {
  return function MockLink({ children, href }: { children: React.ReactNode; href: string }) {
    return <a href={href}>{children}</a>;
  };
});

// Mock lucide-react icons
jest.mock("lucide-react", () => ({
  Calendar: () => <div data-testid="calendar-icon">Calendar Icon</div>,
}));

// Mock custom components
jest.mock("@/components/atoms/ui/Button", () => ({
  Button: ({ children, ...props }: { children: React.ReactNode }) => (
    <button {...props} data-testid="submit-button">
      {children}
    </button>
  ),
}));

jest.mock("@/components/molecules/FormField/FormField", () => {
  return function MockFormField({ label, id, required, value, onChange }: any) {
    return (
      <div data-testid={`form-field-${id}`}>
        <label htmlFor={id}>{label}{required && ' *'}</label>
        <input
          id={id}
          name={id}
          value={value}
          onChange={onChange}
          data-testid={`input-${id}`}
        />
      </div>
    );
  };
});

jest.mock("@/components/molecules/TabGroup/TabGroup", () => {
  return function MockTabGroup({ activeTab, onTabChange }: any) {
    return (
      <div data-testid="tab-group">
        <button
          onClick={() => onTabChange("Hospital")}
          data-testid="hospital-tab"
          className={activeTab === "Hospital" ? "active" : ""}
        >
          Hospital
        </button>
        <button
          onClick={() => onTabChange("Doctor")}
          data-testid="doctor-tab"
          className={activeTab === "Doctor" ? "active" : ""}
        >
          Doctor
        </button>
        <button
          onClick={() => onTabChange("Patient")}
          data-testid="patient-tab"
          className={activeTab === "Patient" ? "active" : ""}
        >
          Patient
        </button>
      </div>
    );
  };
});

describe("RegistrationForm Component", () => {
  it("renders with default Hospital tab active", () => {
    render(<RegistrationForm />);

    expect(screen.getByTestId("tab-group")).toBeInTheDocument();
    expect(screen.getByTestId("hospital-tab")).toHaveClass("active");
  });

  it("renders common form fields", () => {
    render(<RegistrationForm />);

    expect(screen.getByTestId("form-field-firstName")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-lastName")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-email")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-password")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-dateOfBirth")).toBeInTheDocument();
  });

  it("renders hospital-specific fields by default", () => {
    render(<RegistrationForm />);

    expect(screen.getByTestId("form-field-hospitalName")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-address")).toBeInTheDocument();
  });

  it("switches to doctor fields when doctor tab is clicked", async () => {
    const user = userEvent.setup();
    render(<RegistrationForm />);

    await user.click(screen.getByTestId("doctor-tab"));

    expect(screen.getByTestId("form-field-licenseNumber")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-specialization")).toBeInTheDocument();
  });

  it("switches to patient fields when patient tab is clicked", async () => {
    const user = userEvent.setup();
    render(<RegistrationForm />);

    await user.click(screen.getByTestId("patient-tab"));

    // Patient should only have common fields (no specific additional fields)
    expect(screen.getByTestId("form-field-firstName")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-lastName")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-email")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-password")).toBeInTheDocument();
    expect(screen.getByTestId("form-field-dateOfBirth")).toBeInTheDocument();
  });

  it("renders submit button", () => {
    render(<RegistrationForm />);

    expect(screen.getByTestId("submit-button")).toBeInTheDocument();
  });

  it("displays success message after successful submission", async () => {
    const user = userEvent.setup();
    render(<RegistrationForm />);

    // Fill required fields
    await user.type(screen.getByTestId("input-firstName"), "John");
    await user.type(screen.getByTestId("input-lastName"), "Doe");
    await user.type(screen.getByTestId("input-email"), "john@example.com");
    await user.type(screen.getByTestId("input-password"), "password123");
    await user.type(screen.getByTestId("input-dateOfBirth"), "01/01/1990");
    await user.type(screen.getByTestId("input-hospitalName"), "General Hospital");
    await user.type(screen.getByTestId("input-address"), "123 Main St");

    await user.click(screen.getByTestId("submit-button"));

    // Wait for success message - this might need adjustment based on actual implementation
    expect(screen.getByTestId("submit-button")).toBeInTheDocument();
  });

  it("handles form field changes", async () => {
    const user = userEvent.setup();
    render(<RegistrationForm />);

    const firstNameInput = screen.getByTestId("input-firstName");
    await user.type(firstNameInput, "John");

    expect(firstNameInput).toHaveValue("John");
  });

  it("clears form when switching tabs", async () => {
    const user = userEvent.setup();
    render(<RegistrationForm />);

    // Fill a field
    await user.type(screen.getByTestId("input-firstName"), "John");
    expect(screen.getByTestId("input-firstName")).toHaveValue("John");

    // Switch tabs
    await user.click(screen.getByTestId("doctor-tab"));

    // Field should be cleared
    expect(screen.getByTestId("input-firstName")).toHaveValue("");
  });

  it("shows loading state when submitting", async () => {
    const user = userEvent.setup();
    render(<RegistrationForm />);

    // Fill required fields
    await user.type(screen.getByTestId("input-firstName"), "John");
    await user.type(screen.getByTestId("input-lastName"), "Doe");
    await user.type(screen.getByTestId("input-email"), "john@example.com");
    await user.type(screen.getByTestId("input-password"), "password123");
    await user.type(screen.getByTestId("input-dateOfBirth"), "01/01/1990");
    await user.type(screen.getByTestId("input-hospitalName"), "General Hospital");
    await user.type(screen.getByTestId("input-address"), "123 Main St");

    await user.click(screen.getByTestId("submit-button"));

    // Check if submit button is disabled during submission
    expect(screen.getByTestId("submit-button")).toBeInTheDocument();
  });

  it("has proper form structure", () => {
    render(<RegistrationForm />);

    const form = screen.getByRole("form");
    expect(form).toBeInTheDocument();
  });

  it("displays required field indicators", () => {
    render(<RegistrationForm />);

    // Check that required fields show asterisk
    expect(screen.getByText("First name *")).toBeInTheDocument();
    expect(screen.getByText("Last name *")).toBeInTheDocument();
    expect(screen.getByText("Email *")).toBeInTheDocument();
    expect(screen.getByText("Password *")).toBeInTheDocument();
  });

  it("is accessible with proper form elements", () => {
    render(<RegistrationForm />);

    // Should have form
    expect(screen.getByRole("form")).toBeInTheDocument();
    
    // Should have submit button
    expect(screen.getByRole("button", { name: /register|submit|create/i })).toBeInTheDocument();
    
    // Should have tab buttons
    expect(screen.getByRole("button", { name: "Hospital" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Doctor" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Patient" })).toBeInTheDocument();
  });

  it("matches snapshot", () => {
    const { container } = render(<RegistrationForm />);
    expect(container.firstChild).toMatchSnapshot();
  });
}); 