import { requestCreateRoom, useRooms } from "@/api/room";
import { CreateRoomRequest } from "@/pkg/xeejp";
import { useForm } from "react-hook-form";

export default function RoomManagement() {
  const rooms = useRooms("dummy userId");
  const { register, handleSubmit } = useForm<CreateRoomRequest>();

  if (rooms.isLoading) return "loading";
  if (rooms.error) return JSON.stringify(rooms.error);
  if (rooms.isLoadingError) return "loading error";

  const onSubmit = handleSubmit((data) =>
    requestCreateRoom("dummy userId", data)
  );

  return (
    <div>
      <form onSubmit={onSubmit}>
        <label>
          room name
          <input {...register("roomName")} />
        </label>
        <label>
          room key
          <input {...register("roomKey")} />
        </label>
        <label>
          conductor password (not used currently)
          <input {...register("conductorPassword")} />
        </label>
        <input type="submit" />
      </form>
      {Object.entries(rooms.data).map(([roomKey, room]) => (
        <div key={roomKey}>
          <div>key: {roomKey}</div>
          <div>name: {room.name}</div>
        </div>
      ))}
    </div>
  );
}
