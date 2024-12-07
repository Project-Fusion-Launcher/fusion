import { type GameSource } from "./types";

export interface DownloadFinished {
  id: string;
  source: GameSource;
}
