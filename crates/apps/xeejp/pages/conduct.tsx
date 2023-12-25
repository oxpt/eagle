import { clientIdAtom, useUltimatumConductor } from "@/api/gameSocket";
import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { useRouter } from "next/router";
import { v4 as uuidv4 } from "uuid";

const conductorPasswordAtom = atomWithStorage<string>(
  "conductorPassword",
  uuidv4(),
);

const Conduct = () => {
  const router = useRouter();
  if (!router.isReady) return "loading";
  const { roomKey } = router.query;
  return <Inner roomKey={roomKey as string} />;
};

const Inner = ({ roomKey }: { roomKey: string }) => {
  const [channelId] = useAtom(clientIdAtom);
  const [password] = useAtom(conductorPasswordAtom);
  let { state, connectionStatus, sendCommand } = useUltimatumConductor(
    roomKey,
    channelId,
    password,
  );

  return (
    <div>
      <div>connectionStatus: {connectionStatus}</div>
      <div>state: {JSON.stringify(state)}</div>
    </div>
  );
};

export default Conduct;
