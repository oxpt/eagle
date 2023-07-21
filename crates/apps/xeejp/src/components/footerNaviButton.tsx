import React from "react";
import { useRouter } from "next/router";
export interface FooterNaviButtonProps {
  leftTitle: string | null;
  leftDescription: string | null;
  leftURL: string | null;
  rightTitle: string | null;
  rightDescription: string | null;
  rightURL: string | null;
}

import { ArrowRightIcon, ArrowUturnLeftIcon } from "@heroicons/react/20/solid";

export default function FooterNaviButton(props: FooterNaviButtonProps) {
  const router = useRouter();
  const {
    leftTitle,
    leftDescription,
    leftURL,
    rightTitle,
    rightDescription,
    rightURL,
  } = props;

  const handleClick = (
    e: React.MouseEvent<HTMLInputElement>,
    url: string | null
  ) => {
    e.preventDefault();
    url && router.push(url);
  };

  return (
    <div className="mx-auto my-3 grid grid-cols-12 gap-3">
      <div className="col-span-12 sm:col-span-6">
        <input
          type="button"
          id="left"
          name="return"
          defaultValue="left"
          className="peer hidden"
          onClick={(e) => handleClick(e, leftURL)}
        />
        <label
          htmlFor="left"
          className="inline-flex  h-full w-full cursor-pointer  items-center justify-between rounded-lg border border-gray-300 p-4 hover:bg-gray-100 hover:text-gray-700 peer-focus:border-blue-700 peer-focus:text-blue-700"
        >
          <ArrowUturnLeftIcon className="mr-4 h-6 w-6" />
          <div className="block text-right">
            <div className="text-lg font-semibold">{leftTitle}</div>
            <div>{leftDescription}</div>
          </div>
        </label>
      </div>
      <div className="col-span-12 sm:col-span-6">
        <input
          type="button"
          id="right"
          name="mode"
          defaultValue="right"
          className="peer hidden"
          onClick={(e) => handleClick(e, rightURL)}
        />
        <label
          htmlFor="right"
          className="inline-flex  h-full w-full cursor-pointer items-center justify-between  rounded-lg border border-gray-300 p-4 hover:bg-gray-100 hover:text-gray-700 peer-focus:border-blue-700 peer-focus:text-blue-700"
        >
          <div className="block text-left">
            <div className=" text-lg font-semibold">{rightTitle}</div>
            <div>{rightDescription}</div>
          </div>
          <ArrowRightIcon className="ml-4 h-6 w-6" />
        </label>
      </div>
    </div>
  );
}
