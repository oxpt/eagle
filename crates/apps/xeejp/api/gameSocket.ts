import { eagleWebsocketEndpoint } from "@/data/config";
import { PlayerId, UserId } from "@/data/user";
import {
  ClientToServerMessage,
  ServerToClientMessage,
  UltimatumConductor,
  UltimatumConductorCommand,
  UltimatumPlayer,
  UltimatumPlayerCommand,
} from "@/pkg/xeejp";
import { atomFamily, atomWithStorage, useReducerAtom } from "jotai/utils";
import { useCallback, useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";
import { v4 as uuidv4 } from "uuid";
import deepEqual from "fast-deep-equal";
import { Atom, PrimitiveAtom, WritableAtom } from "jotai";

export type RoomRole = "conductor" | "player";
export type ConnectionStatus = "connecting" | "open" | "closing" | "closed";

export const clientIdAtom = atomWithStorage<string>("clientId", uuidv4());

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

export type ClientId = string;

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
    clientId: ClientId;
    playerId: PlayerId;
    playerPassword: string;
  };
} | {
  role: "conduct";
  queryParams: {
    clientId: ClientId;
    conductorPassword: string;
  };
};

function clientStateAtom<C>(
  { gameName, roomKey, clientId, role }: {
    gameName: string;
    roomKey: string;
    clientId: ClientId;
    role: RoomRole;
  },
) {
  return atomWithStorage<State<C>>(
    `gameName:${gameName},roomKey:${roomKey},clientId:${clientId},role:${role}`,
    { ack: null, commands: [] },
  );
}

function useGame<C, V>(
  history: State<C>,
  dispatch: (action: Action<C>) => void,
  roomKey: String,
  roleOption: RoleOption,
) {
  let [view, setView] = useState<V | null>(null);
  const onMessage = useCallback((event: WebSocketEventMap["message"]) => {
    let message = JSON.parse(event.data) as ServerToClientMessage<V>;
    if (message === "Pong") {
      return;
    }
    if ("Ack" in message) {
      dispatch({ type: "Ack", index: message.Ack.index });
    } else if ("Notify" in message) {
      setView(message.Notify.view);
    }
  }, [dispatch, setView]);
  const socket = useWebSocket(
    `${eagleWebsocketEndpoint}/rooms/${roomKey}/${roleOption.role}`,
    {
      share: true, // share a single socket connection with all components using this hook
      retryOnError: true,
      shouldReconnect: (_closeEvent) => true,
      reconnectAttempts: 100,
      // 1, 2, 4, ..., 10 seconds
      reconnectInterval: (attemptNumber) =>
        Math.min(Math.pow(2, attemptNumber) * 1000, 10000),
      queryParams: roleOption.queryParams,
      onOpen: (event) => {
        // TODO: resend all commands on open or reconnect
      },
      onClose: (event) => {
      },
      onError: (event) => {
      },
      onMessage,
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

const ultimatumPlayer = atomFamily(
  (param: { roomKey: string; clientId: string }) =>
    clientStateAtom<UltimatumPlayerCommand>({
      gameName: "ultimatum",
      role: "player",
      ...param,
    }),
  deepEqual,
);

const ultimatumConductor = atomFamily(
  (param: { roomKey: string; clientId: string }) =>
    clientStateAtom<UltimatumConductorCommand>({
      gameName: "ultimatum",
      role: "player",
      ...param,
    }),
  deepEqual,
);

export const useUltimatumPlayer = (
  roomKey: string,
  clientId: ClientId,
  playerId: PlayerId,
  playerPassword: string,
) => {
  const [history, dispatch] = useReducerAtom(
    ultimatumPlayer({ roomKey, clientId }),
    reducer<UltimatumPlayerCommand>,
  );
  return useGame<UltimatumPlayerCommand, UltimatumPlayer>(
    history,
    dispatch,
    roomKey,
    { role: "play", queryParams: { clientId, playerId, playerPassword } },
  );
};

export const useUltimatumConductor = (
  roomKey: string,
  clientId: ClientId,
  conductorPassword: string,
) => {
  const [history, dispatch] = useReducerAtom(
    ultimatumConductor({ roomKey, clientId }),
    reducer<UltimatumConductorCommand>,
  );
  return useGame<UltimatumConductorCommand, UltimatumConductor>(
    history,
    dispatch,
    roomKey,
    { role: "conduct", queryParams: { clientId, conductorPassword } },
  );
};
