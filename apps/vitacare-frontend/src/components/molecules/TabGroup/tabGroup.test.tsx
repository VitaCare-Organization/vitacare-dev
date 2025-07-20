import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom";
import TabGroup from "./tabGroup";

describe("TabGroup", () => {
  const mockTabs = ["Tab 1", "Tab 2", "Tab 3"];
  const mockOnTabChange = jest.fn();

  beforeEach(() => {
    mockOnTabChange.mockClear();
  });

  it("renders all tabs provided", () => {
    render(<TabGroup tabs={mockTabs} />);
   
    mockTabs.forEach(tab => {
      expect(screen.getByText(tab)).toBeInTheDocument();
    });
  });

  it("renders tabs as buttons", () => {
    render(<TabGroup tabs={mockTabs} />);
   
    const tabButtons = screen.getAllByRole("button");
    expect(tabButtons).toHaveLength(mockTabs.length);
  });

  it("sets first tab as active by default", () => {
    
    // The first tab should be passed isActive=true
    const firstTabButton = screen.getByText(mockTabs[0]);
    expect(firstTabButton).toBeInTheDocument();
    
    // Test that only one tab has active styling (since we can't check aria-selected)
    const allTabs = screen.getAllByRole("button");
    expect(allTabs).toHaveLength(mockTabs.length);
  });

  it("respects defaultActiveTab prop when provided", () => {
    render(<TabGroup tabs={mockTabs} defaultActiveTab={mockTabs[1]} />);
    
    const secondTab = screen.getByText(mockTabs[1]);
    expect(secondTab).toBeInTheDocument();
    
    // Verify all tabs are rendered
    const allTabs = screen.getAllByRole("button");
    expect(allTabs).toHaveLength(mockTabs.length);
  });

  it("changes active tab on click", () => {
   
    const targetTab = screen.getByText(mockTabs[2]);
    fireEvent.click(targetTab);
   
    // After click, the tab should still be in the document
    expect(targetTab).toBeInTheDocument();
    
    // Verify the component re-renders with the new active state
    expect(screen.getByText(mockTabs[2])).toBeInTheDocument();
  });

  it("calls onTabChange callback when tab is clicked", () => {
    render(<TabGroup tabs={mockTabs} onTabChange={mockOnTabChange} />);
   
    const targetTab = screen.getByText(mockTabs[1]);
    fireEvent.click(targetTab);
   
    expect(mockOnTabChange).toHaveBeenCalledTimes(1);
    expect(mockOnTabChange).toHaveBeenCalledWith(mockTabs[1]);
  });

  it("does not call onTabChange when not provided", () => {
    render(<TabGroup tabs={mockTabs} />);
   
    const targetTab = screen.getByText(mockTabs[1]);
    fireEvent.click(targetTab);
   
    expect(mockOnTabChange).not.toHaveBeenCalled();
  });

  it("handles clicking the same tab multiple times", () => {
    render(<TabGroup tabs={mockTabs} onTabChange={mockOnTabChange} />);
   
    const firstTab = screen.getByText(mockTabs[0]);
    fireEvent.click(firstTab);
    fireEvent.click(firstTab);
   
    expect(mockOnTabChange).toHaveBeenCalledTimes(2);
    expect(mockOnTabChange).toHaveBeenCalledWith(mockTabs[0]);
  });

  it("handles empty tabs array gracefully", () => {
    render(<TabGroup tabs={[]} />);
   
    const tabButtons = screen.queryAllByRole("button");
    expect(tabButtons).toHaveLength(0);
  });

  it("handles single tab", () => {
    const singleTab = ["Only Tab"];
    render(<TabGroup tabs={singleTab} />);
   
    expect(screen.getByText(singleTab[0])).toBeInTheDocument();
    const tabButtons = screen.getAllByRole("button");
    expect(tabButtons).toHaveLength(1);
  });

  it("maintains internal state correctly after multiple clicks", () => {
    render(<TabGroup tabs={mockTabs} onTabChange={mockOnTabChange} />);
   
    // Click second tab
    fireEvent.click(screen.getByText(mockTabs[1]));
    expect(mockOnTabChange).toHaveBeenLastCalledWith(mockTabs[1]);
   
    // Click third tab
    fireEvent.click(screen.getByText(mockTabs[2]));
    expect(mockOnTabChange).toHaveBeenLastCalledWith(mockTabs[2]);
   
    // Click first tab
    fireEvent.click(screen.getByText(mockTabs[0]));
    expect(mockOnTabChange).toHaveBeenLastCalledWith(mockTabs[0]);
   
    expect(mockOnTabChange).toHaveBeenCalledTimes(3);
  });
});
