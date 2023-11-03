import { clientIdAtom, useUltimatumPlayer } from "@/api/gameSocket";
import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { useRouter } from "next/router";
import { v4 as uuidv4 } from "uuid";

const playerIdAtom = atomWithStorage<string>("playerId", uuidv4());
const playerPasswordAtom = atomWithStorage<string>(
  "playerPassword",
  uuidv4(),
);

const Play = () => {
  const router = useRouter();
  if (!router.isReady) return "loading";
  const { roomKey } = router.query;
  return <Inner roomKey={roomKey as string} />;
};

const Inner = ({ roomKey }: { roomKey: string }) => {
  const [channelId] = useAtom(clientIdAtom);
  const [playerId] = useAtom(playerIdAtom);
  const [playerPassword] = useAtom(playerPasswordAtom);
  let { state, connectionStatus, sendCommand } = useUltimatumPlayer(
    roomKey,
    channelId,
    playerId,
    playerPassword,
  );

  return (
    <div>
      <div>connectionStatus: {connectionStatus}</div>
      <div>state: {JSON.stringify(state)}</div>
    </div>
  );
};

export default Play;
