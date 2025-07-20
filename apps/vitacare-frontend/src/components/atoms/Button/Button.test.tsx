import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import userEvent from "@testing-library/user-event";
import { Button } from "./button";

describe("Button Component", () => {
  it("renders correctly", () => {
    render(<Button>Click Me</Button>);
    expect(
      screen.getByRole("button", { name: /click me/i })
    ).toBeInTheDocument();
  });

  it("displays children correctly", () => {
    render(<Button>Submit</Button>);
    expect(screen.getByText("Submit")).toBeInTheDocument();
  });

  it("applies primary styles by default", () => {
    render(<Button>Primary Button</Button>);
    const button = screen.getByRole("button", { name: /primary button/i });
    expect(button).toHaveClass("bg-primary text-white");
  });

  it("applies outline variant styles", () => {
    render(<Button variant="outline">Outline Button</Button>);
    const button = screen.getByRole("button", { name: /outline button/i });
    expect(button).toHaveClass("border-primary text-primary");
  });

  it("applies full width when fullWidth is true", () => {
    render(<Button fullWidth>Full Width Button</Button>);
    const button = screen.getByRole("button", { name: /full width button/i });
    expect(button).toHaveClass("w-full");
  });

  it("shows a loading spinner when isLoading is true", () => {
    render(<Button isLoading>Loading...</Button>);
    expect(screen.getByRole("button")).toHaveClass(
      "opacity-50 cursor-not-allowed"
    );
    expect(screen.getByRole("button")).toContainHTML(
      '<div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"></div>'
    );
  });

  it("disables the button when isLoading is true", () => {
    render(<Button isLoading>Loading Button</Button>);
    expect(screen.getByRole("button")).toBeDisabled();
  });

  it("disables the button when disabled prop is true", () => {
    render(<Button disabled>Disabled Button</Button>);
    expect(screen.getByRole("button")).toBeDisabled();
  });

  it("triggers onClick function when clicked", async () => {
    const handleClick = jest.fn();
    render(<Button onClick={handleClick}>Click Me</Button>);
    const button = screen.getByRole("button", { name: /click me/i });

    await userEvent.click(button);
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it("does not trigger onClick when disabled", async () => {
    const handleClick = jest.fn();
    render(
      <Button onClick={handleClick} disabled>
        Disabled Button
      </Button>
    );
    const button = screen.getByRole("button", { name: /disabled button/i });

    await userEvent.click(button);
    expect(handleClick).not.toHaveBeenCalled();
  });
});
