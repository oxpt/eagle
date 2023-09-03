import { useRooms } from "@/api/room";
import { UserId } from "@/data/user";

const RoomsManagement = ({ userId }: { userId: UserId }) => {
  let {  = useRooms(userId);
  return (
    <div>
      {rooms.map((room) => <div>{room.name}</div>)}
    </div>
  );
};

export default RoomsManagement;
