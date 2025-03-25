import React from "react";
import { Button } from "@/components/atoms/Button/Button";
import Image from "next/image";
import HeroImage from "@/assets/Vector.svg";

export default function Hero() {
  return (
    <div className="min-h-screen flex justify-center items-center">
      <div className="flex flex-col lg:flex-row gap-5 justify-between px-4 md:px-10 xl:px-20  items-center 2xl:max-w-[1440px] 2xl:mx-auto">
        <div className=" flex flex-col sm:text-center lg:text-left  sm:justify-center sm:items-center lg:items-start gap-3 lg:w-1/2">
          <h1 className="font-bold text-[#020817] text-3xl md:text-4xl md:leading-8 xl:text-6xl lg:leading-16">
            Secure Healthcare on the Blockchain
          </h1>
          <p className="text-[#64748B] sm:text-center lg:text-left font-normal text-sm md:text-xl md:leading-7 w-5/6">
            VitaCare connects hospitals, doctors, and patients with secure,
            blockchain-verified medical records and seamless appointment
            management.
          </p>
          <div className="flex flex-col min-[400px]:flex-row  gap-2">
            <Button
              className="text-[#F8FAFC] font-medium bg-[#0096CC] hover:brightness-110
            rounded-[6px] py-3 px-8"
            >
              <p> Get Started</p>
            </Button>
            <Button
              className="  border 
            border-[#E2E8F0] bg-white rounded-[6px] py-3 px-8 hover:brightness-90"
            >
              <p className="font-medium text-sm text-[#020817]"> Learn More</p>
            </Button>
          </div>
        </div>
        <div className=" lg:w-1/2 flex justify-end items-end">
          <Image src={HeroImage} alt="" />
        </div>
      </div>
    </div>
  );
}
