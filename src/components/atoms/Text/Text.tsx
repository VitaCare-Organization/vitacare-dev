interface TextProps {
    children: string;
    className?: string;
  }
  
  export function Heading({ children }: TextProps) {
    return <h3 className="text-lg font-semibold text-gray-900">{children}</h3>;
  }
  
  export function Description({ children }: TextProps) {
    return <p className="text-gray-600">{children}</p>;
  }
  