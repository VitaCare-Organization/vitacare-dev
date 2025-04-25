import { ReactNode } from "react";

interface IconProps {
  children: ReactNode;
}

export default function Icon({ children }: IconProps) {
  return <div className="text-blue-500 text-3xl">{children}</div>;
}
