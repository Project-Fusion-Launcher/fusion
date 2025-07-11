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

export type GameStatus =
  | "installed"
  | "notInstalled"
  | "downloading"
  | "installing"
  | "uninstalling";
export type GameSource = "legacyGames" | "itchio";

export interface GameFilters {
  query?: string;
}

export type GameFiltersStatus = "all" | GameStatus;

export interface GameVersion {
  id: string;
  name: string;
  external: boolean;
}

export interface GameVersionInfo {
  installSize: number;
  downloadSize: number;
}

export interface DownloadOptions {
  installLocation: string;
}

export interface DownloadItem {
  gameId: string;
  gameSource: GameSource;
  gameTitle: string;
  downloadSize: number;
  downloaded: number;
}
