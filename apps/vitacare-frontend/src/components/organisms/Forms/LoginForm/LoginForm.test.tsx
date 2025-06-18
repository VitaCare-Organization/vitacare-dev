import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import LoginForm from "./LoginForm";

// Mock Next.js components and dependencies
jest.mock("next/link", () => {
  return function MockLink({ children, href }: { children: React.ReactNode; href: string }) {
    return <a href={href}>{children}</a>;
  };
});

jest.mock("next/image", () => {
  return function MockImage({ src, alt, width, height }: { src: string; alt: string; width?: number; height?: number }) {
    return <img src={src} alt={alt} width={width} height={height} data-testid="badge-icon" />;
  };
});

// Mock form dependencies
jest.mock("@hookform/resolvers/zod", () => ({
  zodResolver: jest.fn(() => () => ({})),
}));

jest.mock("react-hook-form", () => ({
  useForm: () => ({
    register: jest.fn(() => ({})),
    handleSubmit: jest.fn((fn) => (e: Event) => {
      e.preventDefault();
      fn({});
    }),
    setValue: jest.fn(),
    watch: jest.fn(() => "Patient"),
    formState: { errors: {} },
  }),
}));

// Mock custom components
jest.mock("@/components/atoms/Input/Input", () => {
  return function MockInput(props: any) {
    return <input {...props} data-testid={`input-${props.label?.toLowerCase()}`} />;
  };
});

jest.mock("@/components/atoms/Input/PasswordInput", () => {
  return function MockPasswordInput(props: any) {
    return <input type="password" {...props} data-testid="password-input" />;
  };
});

jest.mock("@/components/atoms/Button/Button", () => ({
  Button: ({ children, ...props }: { children: React.ReactNode }) => (
    <button {...props} data-testid="submit-button">
      {children}
    </button>
  ),
}));

jest.mock("@/components/molecules/AccountType/AccountTypeSelector", () => {
  return function MockAccountTypeSelector() {
    return <div data-testid="account-type-selector">Account Type Selector</div>;
  };
});

// Mock assets
jest.mock("@/components/atoms/icons/badgeicon.svg", () => "mock-badge-icon.svg");

describe("LoginForm Component", () => {
  it("renders the welcome heading and description", () => {
    render(<LoginForm />);

    expect(screen.getByRole("heading", { name: /welcome back/i })).toBeInTheDocument();
    expect(screen.getByText("Sign in to your VitaCare account")).toBeInTheDocument();
  });

  it("displays the badge icon", () => {
    render(<LoginForm />);

    const badgeIcon = screen.getByTestId("badge-icon");
    expect(badgeIcon).toBeInTheDocument();
    expect(badgeIcon).toHaveAttribute("src", "mock-badge-icon.svg");
    expect(badgeIcon).toHaveAttribute("alt", "badge");
  });

  it("renders the sign in form heading", () => {
    render(<LoginForm />);

    expect(screen.getByRole("heading", { name: /sign in/i })).toBeInTheDocument();
    expect(screen.getByText("Enter your credentials to access your account")).toBeInTheDocument();
  });

  it("renders email input field", () => {
    render(<LoginForm />);

    const emailInput = screen.getByTestId("input-email");
    expect(emailInput).toBeInTheDocument();
  });

  it("renders password input field", () => {
    render(<LoginForm />);

    const passwordInput = screen.getByTestId("password-input");
    expect(passwordInput).toBeInTheDocument();
  });

  it("renders account type selector", () => {
    render(<LoginForm />);

    expect(screen.getByTestId("account-type-selector")).toBeInTheDocument();
  });

  it("displays forgot password link", () => {
    render(<LoginForm />);

    const forgotPasswordLink = screen.getByRole("link", { name: /forgot password/i });
    expect(forgotPasswordLink).toBeInTheDocument();
    expect(forgotPasswordLink).toHaveAttribute("href", "#");
  });

  it("renders submit button", () => {
    render(<LoginForm />);

    const submitButton = screen.getByTestId("submit-button");
    expect(submitButton).toBeInTheDocument();
    expect(submitButton).toHaveAttribute("type", "submit");
  });

  it("displays sign up link", () => {
    render(<LoginForm />);

    expect(screen.getByText("Don't have an account?")).toBeInTheDocument();
    const signUpLink = screen.getByRole("link", { name: /sign up/i });
    expect(signUpLink).toBeInTheDocument();
    expect(signUpLink).toHaveAttribute("href", "/");
  });

  it("has correct form structure", () => {
    render(<LoginForm />);

    const form = screen.getByRole("form");
    expect(form).toBeInTheDocument();
  });

  it("main container has correct styling", () => {
    render(<LoginForm />);

    const mainContainer = screen.getByRole("heading", { name: /welcome back/i }).closest(".min-h-screen");
    expect(mainContainer).toHaveClass("min-h-screen flex justify-center items-center w-full");
  });

  it("form container has correct styling", () => {
    render(<LoginForm />);

    const form = screen.getByRole("form");
    expect(form).toHaveClass("w-full md:max-w-[360px] md:mx-auto space-y-4 bg-white h-fit p-3 md:p-6 rounded-lg border border-[#E2E8F0] shadow-[0px_1px_2px_#0000000D]");
  });

  it("submit button has correct styling", () => {
    render(<LoginForm />);

    const submitButton = screen.getByTestId("submit-button");
    expect(submitButton).toHaveClass("rounded-[6px] bg-[#0096CC] hover:brightness-110 py-3 text-[#18181B]");
  });

  it("forgot password link has correct styling", () => {
    render(<LoginForm />);

    const forgotPasswordLink = screen.getByRole("link", { name: /forgot password/i });
    expect(forgotPasswordLink).toHaveClass("text-[#0096CC] text-sm hover:underline");
  });

  it("sign up link has correct styling", () => {
    render(<LoginForm />);

    const signUpLink = screen.getByRole("link", { name: /sign up/i });
    expect(signUpLink).toHaveClass("text-sm hover:underline text-[#0096CC]");
  });

  it("is accessible with proper form structure", () => {
    render(<LoginForm />);

    // Should have form landmark
    expect(screen.getByRole("form")).toBeInTheDocument();
    
    // Should have proper headings
    expect(screen.getByRole("heading", { name: /welcome back/i })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: /sign in/i })).toBeInTheDocument();
    
    // Should have submit button
    expect(screen.getByRole("button", { name: /sign in/i })).toBeInTheDocument();
  });

  it("welcome section has correct layout", () => {
    render(<LoginForm />);

    const welcomeSection = screen.getByRole("heading", { name: /welcome back/i }).closest(".text-center");
    expect(welcomeSection).toHaveClass("text-center flex flex-col gap-2");
  });

  it("matches snapshot", () => {
    const { container } = render(<LoginForm />);
    expect(container.firstChild).toMatchSnapshot();
  });
}); 