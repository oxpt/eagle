import Image from "next/image";
import { useState } from "react";
import EnterUUID from "@/components/visitors/enterUUID";
import { useForm } from "react-hook-form";
import { useRouter } from "next/router";

type JoinForm = {
  roomKey: string;
  playerId: string | undefined;
};

export default function TopPage() {
  const router = useRouter();
  const { register, handleSubmit } = useForm<JoinForm>();
  const onSubmit = handleSubmit((data: JoinForm) => {
    if (data.playerId && data.playerId !== "") {
      router.push(`/play?roomKey=${data.roomKey}&playerId=${data.playerId}`);
    } else {
      router.push(`/play?roomKey=${data.roomKey}`);
    }
  });

  return (
    <div className="mx-auto grid grid-cols-12 gap-3">
      <Image
        src="/logo.svg"
        width={549}
        height={926}
        alt="Logo"
        priority={true}
        className="col-span-12 mx-auto mb-[10svh] mt-[15svh] h-[40svh] w-auto items-center"
      />
      <div className="col-span-1 items-center text-center sm:col-span-2 md:col-span-3 lg:col-span-4">
        {" "}
      </div>
      <div className="col-span-10  mb-[15svh] w-full items-center justify-center text-center sm:col-span-8 md:col-span-6 lg:col-span-4">
        <div className="group relative">
          <form onSubmit={onSubmit}>
            <input
              type="text"
              id="room_name"
              className="block w-full rounded-lg border border-blue-300 bg-gray-50 p-2.5 text-base text-gray-900"
              placeholder="ルーム名を入力"
              required
              {...register("roomKey")}
            />
            <input
              type="text"
              id="player_id"
              className="block w-full rounded-lg border border-blue-300 bg-gray-50 p-2.5 text-base text-gray-900"
              placeholder="プレイヤーID"
              required
              {...register("playerId")}
            />
            <button
              type="submit"
              className="absolute right-[0.5px] top-[0.75px] bg-inherit p-[2.25px]  text-base text-gray-300 hover:text-blue-500 focus:text-blue-500 group-hover:text-blue-500"
            >
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
