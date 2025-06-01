import type { GameSource, GameStatus } from "../bindings";

export type GameFiltersStatus = "all" | GameStatus;

export interface DownloadItem {
  gameId: string;
  gameSource: GameSource;
  gameTitle: string;
  downloadSize: number;
  downloaded: number;
}
