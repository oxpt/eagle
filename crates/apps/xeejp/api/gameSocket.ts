import { eagleWebsocketEndpoint } from "@/data/config";
import { PlayerId, UserId } from "@/data/user";
import {
  ClientToServerMessage,
  PlayRequest,
  ServerToClientMessage,
  UltimatumPlayer,
  UltimatumPlayerCommand,
} from "@/pkg/xeejp";
import { atomWithStorage, useReducerAtom } from "jotai/utils";
import { useState } from "react";
import useWebSocket from "react-use-websocket";
import { QueryParams } from "react-use-websocket/dist/lib/types";

export type RoomRole = "conductor" | "player";
export type ConnectionStatus = "connecting" | "open" | "closing" | "closed";

const toPath = (role: RoomRole) => role === "conductor" ? "conduct" : "play";
const connectionStatus = (status: number): ConnectionStatus | undefined => {
  switch (status) {
    case 0:
      return "connecting";
    case 1:
      return "open";
    case 2:
      return "closing";
    case 3:
      return "closed";
  }
};

export type ChannelId = string;

type State<C> = {
  ack: number | null;
  commands: C[];
};
type Action<C> = { type: "sendCommand"; command: C } | {
  type: "Ack";
  index: number;
};
function reducer<C>(state: State<C>, action: Action<C>): State<C> {
  switch (action.type) {
    case "sendCommand":
      return { ...state, commands: [...state.commands, action.command] };
    case "Ack":
      return { ...state, ack: action.index };
  }
}

type RoleOption = {
  role: "play";
  queryParams: {
    playerId: PlayerId;
    playerPassword: string;
  };
} | {
  role: "conduct";
  queryParams: {
    conductorPassword: string;
  };
};

function useGame<C, V>(
  conductor: UserId,
  roomKey: String,
  channelId: ChannelId,
  roleOption: RoleOption,
) {
  let [view, setView] = useState<V | null>(null);
  const channelStateAtom = atomWithStorage<State<C>>(
    `channelState/${conductor}/${roomKey}/${channelId}`,
    { ack: null, commands: [] },
  );
  const [history, dispatch] = useReducerAtom(channelStateAtom, reducer<C>);
  const socket = useWebSocket(
    `${eagleWebsocketEndpoint}/users/${conductor}/rooms/${roomKey}/channels/${channelId}/${roleOption.role}`,
    {
      share: true, // share a single socket connection with all components using this hook
      retryOnError: true,
      shouldReconnect: (_closeEvent) => true,
      reconnectAttempts: 100,
      // 1, 2, 4, ..., 10 seconds
      reconnectInterval: (attemptNumber) =>
        Math.min(Math.pow(2, attemptNumber) * 1000, 10000),

      queryParams: roleOption.queryParams,
      onOpen: (_event) => {
        // TODO: resend (all) history
      },
      onMessage: (event) => {
        const message = event.data as ServerToClientMessage<V>;
        if (message === "Pong") {
          return;
        }
        if ("Ack" in message) {
          dispatch({ type: "Ack", index: message.Ack.index });
        } else if ("Notify" in message) {
          setView(message.Notify.view);
        }
      },
    },
  );
  return {
    state: view,
    connectionStatus: connectionStatus(socket.readyState),
    sendCommand: (command: C) => {
      let message: ClientToServerMessage<C> = {
        Command: { index: history.commands.length, command },
      };
      socket.sendJsonMessage(message);
      dispatch({ type: "sendCommand", command });
    },
  };
}

export const useUltimatumPlayer = (
  conductor: UserId,
  roomKey: String,
  channelId: ChannelId,
  playerId: PlayerId,
  playerPassword: string,
) => {
  return useGame<UltimatumPlayerCommand, UltimatumPlayer>(
    conductor,
    roomKey,
    channelId,
    { role: "play", queryParams: { playerId, playerPassword } },
  );
};

export const useUltimatumConductor = (
  conductor: UserId,
  roomKey: String,
  channelId: ChannelId,
  conductorPassword: string,
) => {
  return useGame<UltimatumPlayerCommand, UltimatumPlayer>(
    conductor,
    roomKey,
    channelId,
    { role: "conduct", queryParams: { conductorPassword } },
  );
};
