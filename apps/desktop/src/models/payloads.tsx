import type { GameStatus } from "./types";
import { type GameSource } from "./types";

export interface DownloadFinishedPayload {
  gameId: string;
  gameSource: GameSource;
}

export interface GameUninstalledPayload {
  gameId: string;
  gameSource: GameSource;
}

export interface GameFiltersPayload {
  query?: string;
  status?: GameFiltersStatus;
}

export type GameFiltersStatus = "all" | GameStatus;
