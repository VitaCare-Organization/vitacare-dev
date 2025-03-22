import { Shield } from "lucide-react";

export default function Footer() {
  return (
    <footer className="flex flex-col md:flex-row justify-between items-center px-8 md:px-20 py-5 md:py-7 border-t border-[#E2E8F0] text-center md:text-left">
      <div className="flex items-center space-x-2">
        <Shield className="text-[#0096CC]" />
        <p className="font-bold text-xl text-[#020817]">VitaCare</p>
      </div>

      <p className="text-[#64748B] mt-2 md:mt-0">
        &copy; {new Date().getFullYear()} VitaCare. All rights reserved. Powered
        by Stellar blockchain.
      </p>

      <ul className="flex space-x-4 text-[#64748B] mt-2 md:mt-0 *:cursor-pointer">
        <li>Terms</li>
        <li>Privacy</li>
        <li>Contact</li>
      </ul>
    </footer>
  );
}
