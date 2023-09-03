import { requestCreateRoom, useRooms } from "@/api/room";
import { userIdAtom } from "@/data/user";
import { CreateRoomRequest } from "@/pkg/xeejp";
import { useAtom } from "jotai";
import { useCallback, useEffect } from "react";
import { v4 } from "uuid";

export default function HostIndex() {
  let [userIdOrNull, setUserId] = useAtom(userIdAtom);

  // FIXME: use authenticated user id
  useEffect(() => {
    if (userIdOrNull === null) {
      setUserId(v4());
    }
  }, []);
  let createRoom = useCallback(async () => {
    let body: CreateRoomRequest = {
      roomName: "test",
      roomKey: "test",
      conductorPassword: "test",
    };
    await requestCreateRoom(userIdOrNull!!, body);
  }, []);
  let rooms = useRooms(userIdOrNull!!);

  return (
    <div>
      {userIdOrNull
        ? <button onClick={createRoom}></button>
        : <div>not logged in</div>}
    </div>
  );
}
