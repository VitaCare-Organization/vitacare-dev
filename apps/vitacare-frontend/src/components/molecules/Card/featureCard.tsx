
import Icon from "@/components/atoms/Icon/icon";
import { Description, Heading } from "@/components/atoms/Text/text";
import { ReactNode } from "react";

interface FeatureCardProps {
  icon: ReactNode;
  title: string;
  description: string;
}

export default function FeatureCard({ icon, title, description }: FeatureCardProps) {
  return (
    <div className="flex flex-col items-center p-5  bg-[#F1F5F966] rounded-md shadow-sm border border-[#E2E8F0] text-center space-y-2">
      <Icon>{icon}</Icon>
      <Heading>{title}</Heading>
      <Description>{description}</Description>
    </div>
  );
}
