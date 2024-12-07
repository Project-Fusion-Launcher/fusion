export interface Game {
  id: string;
  title: string;
  source: GameSource;
  developer?: string;
  status: GameStatus;
}

export type GameStatus = "installed" | "notInstalled";
export type GameSource = "legacyGames" | "itchio";
