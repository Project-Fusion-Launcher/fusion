import { type GameSource } from "./types";

export interface DownloadFinishedPayload {
  gameId: string;
  gameSource: GameSource;
}

export interface GameUninstalledPayload {
  gameId: string;
  gameSource: GameSource;
}
