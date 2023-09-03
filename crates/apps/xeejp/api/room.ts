import { eagleServerEndpoint } from "@/data/config";
import { UserId } from "@/data/user";
import { CreateRoomRequest, Room, Rooms, RoomsResponse } from "@/pkg/xeejp";
import { useQuery } from "@tanstack/react-query";

export async function requestCreateRoom(
  userId: UserId,
  body: CreateRoomRequest,
): Promise<{}> {
  let res = await fetch(`${eagleServerEndpoint}/users/${userId}/rooms`, {
    method: "POST",
    body: JSON.stringify(body),
  });
  return res.json();
}

const roomsQueryKey = (userId: UserId) => ["rooms", userId];

export const useRooms = (userId: UserId) => {
  return useQuery(roomsQueryKey(userId), async () => {
    let res = await fetch(`${eagleServerEndpoint}/users/${userId}/rooms`);
    let json: RoomsResponse = await res.json();
    return json.rooms;
  });
};
