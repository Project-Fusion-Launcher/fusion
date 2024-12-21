import { type GameSource } from "./types";

export interface GameUninstalledPayload {
  gameId: string;
  gameSource: GameSource;
}
