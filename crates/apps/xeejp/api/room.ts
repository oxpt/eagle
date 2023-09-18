import { eagleServerEndpoint } from "@/data/config";
import { UserId } from "@/data/user";
import { CreateRoomRequest, Room, Rooms, RoomsResponse } from "@/pkg/xeejp";
import { useQuery } from "@tanstack/react-query";
import { queryClient } from "./client";

export async function requestCreateRoom(
  userId: UserId,
  body: CreateRoomRequest,
): Promise<{}> {
  let res = await fetch(`${eagleServerEndpoint}/users/${userId}/rooms`, {
    method: "POST",
    body: JSON.stringify(body),
  });
  queryClient.invalidateQueries(roomsQueryKey(userId));
  return res.text();
}

const roomsQueryKey = (userId: UserId) => ["rooms", userId];

export const useRooms = (userId: UserId) => {
  return useQuery(roomsQueryKey(userId), async () => {
    let res = await fetch(`${eagleServerEndpoint}/users/${userId}/rooms`, {
      headers: {
        "Content-Type": "application/json",
      },
    });
    let json: RoomsResponse = await res.json();
    return json.rooms;
  });
};
