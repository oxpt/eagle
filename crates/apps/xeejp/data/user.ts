import { atomWithStorage } from "jotai/utils";

export type UserId = string;
export const userIdAtom = atomWithStorage<UserId | null>("userId", null);
