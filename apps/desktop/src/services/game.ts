import { invoke } from "@tauri-apps/api/core";
import type {
  DownloadOptions,
  Game,
  GameFilters,
  GameVersion,
  GameVersionInfo,
} from "../models/types";

export async function getGames(
  refetch = false,
  filters?: GameFilters,
): Promise<Game[]> {
  return invoke<Game[]>("get_games", { refetch, filters });
}

export async function hideGame(game: Game): Promise<void> {
  invoke<void>("hide_game", {
    gameId: game.id,
    gameSource: game.source,
  });
}

export async function uninstallGame(game: Game): Promise<void> {
  invoke<void>("uninstall_game", {
    gameId: game.id,
    gameSource: game.source,
  });
}

export async function fetchGameVersions(game: Game): Promise<GameVersion[]> {
  return invoke<GameVersion[]>("fetch_game_versions", {
    gameId: game.id,
    gameSource: game.source,
  });
}

export async function fetchGameVersionInfo(
  game: Game,
  version: GameVersion,
): Promise<GameVersionInfo> {
  return invoke<GameVersionInfo>("fetch_game_version_info", {
    gameId: game.id,
    gameSource: game.source,
    versionId: version.id,
  });
}

export async function downloadGame(
  game: Game,
  version: GameVersion,
  options: DownloadOptions,
): Promise<void> {
  invoke<void>("download_game", {
    gameId: game.id,
    gameSource: game.source,
    versionId: version.id,
    downloadOptions: options,
  });
}

export async function launchGame(game: Game): Promise<void> {
  invoke<void>("launch_game", {
    gameId: game.id,
    gameSource: game.source,
  });
}
