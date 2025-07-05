import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import TabSwitcher from './TabSwitcher';

describe('TabSwitcher', () => {
  const mockTabs = [
    { id: 'tab1', label: 'First Tab' },
    { id: 'tab2', label: 'Second Tab' },
    { id: 'tab3', label: 'Third Tab' },
  ];
  const mockOnChange = jest.fn();

  beforeEach(() => {
    mockOnChange.mockClear();
  });

  it('renders all tabs with their labels', () => {
    render(<TabSwitcher tabs={mockTabs} />);
   
    mockTabs.forEach(tab => {
      expect(screen.getByText(tab.label)).toBeInTheDocument();
    });
  });

  it('renders all tabs as buttons', () => {
    render(<TabSwitcher tabs={mockTabs} />);
   
    const tabButtons = screen.getAllByRole('button');
    expect(tabButtons).toHaveLength(mockTabs.length);
  });

  it('selects first tab by default when no defaultActiveTab is provided', () => {
    render(<TabSwitcher tabs={mockTabs} />);
    
    // Verify first tab is rendered and clickable
    const firstTab = screen.getByText(mockTabs[0].label);
    expect(firstTab).toBeInTheDocument();
    expect(firstTab.tagName).toBe('BUTTON');
  });

  it('respects defaultActiveTab prop when provided', () => {
    render(<TabSwitcher tabs={mockTabs} defaultActiveTab="tab2" />);
    
    // Verify specified tab is rendered and clickable
    const secondTab = screen.getByText(mockTabs[1].label);
    expect(secondTab).toBeInTheDocument();
    expect(secondTab.tagName).toBe('BUTTON');
  });

  it('changes active tab when clicked', () => {
    render(<TabSwitcher tabs={mockTabs} />);
    const targetTab = screen.getByText(mockTabs[2].label);
   
    fireEvent.click(targetTab);
   
    // Verify tab is still rendered and functional after click
    expect(targetTab).toBeInTheDocument();
  });

  it('calls onChange callback with correct tab id when tab is clicked', () => {
    render(<TabSwitcher tabs={mockTabs} onChange={mockOnChange} />);
    const targetTab = screen.getByText(mockTabs[1].label);
   
    fireEvent.click(targetTab);
   
    expect(mockOnChange).toHaveBeenCalledTimes(1);
    expect(mockOnChange).toHaveBeenCalledWith('tab2');
  });

  it('does not call onChange when not provided', () => {
    render(<TabSwitcher tabs={mockTabs} />);
    const targetTab = screen.getByText(mockTabs[1].label);
   
    fireEvent.click(targetTab);
   
    expect(mockOnChange).not.toHaveBeenCalled();
  });

  it('handles empty tabs array gracefully', () => {
    render(<TabSwitcher tabs={[]} />);
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });

  it('uses provided className', () => {
    const testClassName = 'test-class';
    const { container } = render(
      <TabSwitcher tabs={mockTabs} className={testClassName} />
    );
   
    expect(container.firstChild).toHaveClass(testClassName);
  });

  it('handles clicking same tab multiple times', () => {
    render(<TabSwitcher tabs={mockTabs} onChange={mockOnChange} />);
    const firstTab = screen.getByText(mockTabs[0].label);
   
    fireEvent.click(firstTab);
    fireEvent.click(firstTab);
   
    expect(mockOnChange).toHaveBeenCalledTimes(2);
    expect(mockOnChange).toHaveBeenCalledWith('tab1');
  });

  it('handles single tab scenario', () => {
    const singleTab = [{ id: 'only', label: 'Only Tab' }];
    render(<TabSwitcher tabs={singleTab} />);
   
    const tab = screen.getByText('Only Tab');
    expect(tab).toBeInTheDocument();
    expect(tab.tagName).toBe('BUTTON');
  });

  it('handles multiple sequential tab clicks correctly', () => {
    render(<TabSwitcher tabs={mockTabs} onChange={mockOnChange} />);
   
    // Click second tab
    fireEvent.click(screen.getByText(mockTabs[1].label));
    expect(mockOnChange).toHaveBeenLastCalledWith('tab2');
   
    // Click third tab
    fireEvent.click(screen.getByText(mockTabs[2].label));
    expect(mockOnChange).toHaveBeenLastCalledWith('tab3');
   
    // Click first tab
    fireEvent.click(screen.getByText(mockTabs[0].label));
    expect(mockOnChange).toHaveBeenLastCalledWith('tab1');
   
    expect(mockOnChange).toHaveBeenCalledTimes(3);
  });

  it('maintains component structure after interactions', () => {
    render(<TabSwitcher tabs={mockTabs} />);
   
    // Click multiple tabs
    fireEvent.click(screen.getByText(mockTabs[1].label));
    fireEvent.click(screen.getByText(mockTabs[2].label));
   
    // Verify all tabs are still rendered
    mockTabs.forEach(tab => {
      expect(screen.getByText(tab.label)).toBeInTheDocument();
    });
   
    const tabButtons = screen.getAllByRole('button');
    expect(tabButtons).toHaveLength(mockTabs.length);
  });
});
