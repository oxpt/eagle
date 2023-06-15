import Image from 'next/image';
import { useState } from 'react';
import EnterUUID from './enterUUID';

export default function TopPage() {
  const [open, setOpen] = useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(!open);
  };

  return (
    <>
      <Image
        src="/logo.png"
        width={549}
        height={926}
        alt="Logo"
        priority={false}
        className="mx-auto mb-[10svh] mt-[20svh] w-auto h-[40svh] max-w-screen-xl items-center"
      />

      <div className="mx-auto mb-4 max-w-sm items-center px-2 md:max-w-xs">
        <div className="group relative">
          <input
            type="text"
            id="room_name"
            className=" block w-full rounded-lg border border-blue-300 bg-gray-50 p-2.5  text-base text-gray-900 focus:outline-blue-500"
            placeholder="ルーム名を入力してください。"
            required
          />
          <button
            type="button"
            onClick={handleClickOpen}
            className="absolute right-[0.5px] top-[0.75px] bg-inherit p-[2.25px]  text-base text-gray-300 hover:text-blue-500 focus:text-blue-500 group-hover:text-blue-500"
          >
            <svg
              className="h-10 w-10"
              fill="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
              aria-hidden="true"
            >
              <path
                clipRule="evenodd"
                fillRule="evenodd"
                d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm4.28 10.28a.75.75 0 000-1.06l-3-3a.75.75 0 10-1.06 1.06l1.72 1.72H8.25a.75.75 0 000 1.5h5.69l-1.72 1.72a.75.75 0 101.06 1.06l3-3z"
              />
            </svg>
          </button>
        </div>
      </div>
      <div className="mb-[20svh]" />
      <EnterUUID open={open} onClose={handleClose} />
    </>
  );
}
