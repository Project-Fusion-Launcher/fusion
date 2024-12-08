export interface Game {
  id: string;
  title: string;
  source: GameSource;
  developer?: string;
  status: GameStatus;
  path?: string;
  favorite: boolean;
  hidden: boolean;
}

export type GameStatus = "installed" | "notInstalled";
export type GameSource = "legacyGames" | "itchio";
