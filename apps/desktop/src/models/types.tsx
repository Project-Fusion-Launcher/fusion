export interface Game {
  id: string;
  title: string;
  source: GameSource;
  developer?: string;
  status: GameStatus;
  path?: string;
  favorite: boolean;
  hidden: boolean;
  coverUrl?: string;
}

export type GameStatus = "installed" | "notInstalled";
export type GameSource = "legacyGames" | "itchio";

export interface GameFilters {
  query?: string;
}

export type GameFiltersStatus = "all" | GameStatus;

export interface GameVersion {
  id: string;
  gameId: string;
  source: string;
  name: string;
  downloadSize: number;
}

export interface VersionDownloadInfo {
  installSize: number;
}
