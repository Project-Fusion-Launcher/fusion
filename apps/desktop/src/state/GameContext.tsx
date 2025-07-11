import type { JSXElement } from "solid-js";
import { createContext, createEffect, onCleanup } from "solid-js";
import type { DownloadItem, Game, GameFilters } from "../models/types";
import type { SetStoreFunction } from "solid-js/store";
import { createStore, produce } from "solid-js/store";
import { listen } from "@tauri-apps/api/event";
import { getGames as getGamesFromBackend } from "../services/game";

export const GameContext = createContext<{
  state: {
    games: Game[];
    downloadQueue: DownloadItem[];
    externalDownloads: DownloadItem[];
    completedDownloads: DownloadItem[];
    total: number;
    installed: number;
    getGames: (refetch?: boolean, filters?: GameFilters) => void;
  };
  setState: SetStoreFunction<{
    games: Game[];
    downloadQueue: DownloadItem[];
    externalDownloads: DownloadItem[];
    completedDownloads: DownloadItem[];
    total: number;
    installed: number;
  }>;
}>({
  state: {
    games: [],
    downloadQueue: [],
    externalDownloads: [],
    completedDownloads: [],
    total: 0,
    installed: 0,
    getGames: () => {},
  },
  setState: () => {},
});

interface StateProps {
  children: JSXElement;
}

const ContextProvider = (props: StateProps) => {
  const [state, setState] = createStore({
    games: [] as Game[],
    downloadQueue: [] as DownloadItem[],
    externalDownloads: [] as DownloadItem[],
    completedDownloads: [] as DownloadItem[],
    total: 0,
    installed: 0,
    getGames,
  });

  createEffect(() => {
    refreshGameCount();
  });

  function refreshGameCount() {
    setState("total", state.games.length);
    setState(
      "installed",
      state.games.filter((g) => g.status === "installed").length,
    );
  }

  function getGames(refetch = false, filters?: GameFilters) {
    getGamesFromBackend(refetch, filters).then((games) => {
      setState("games", games);
    });
  }

  const gameHiddenUnlisten = listen<Game>("game-hidden", (event) => {
    const game = event.payload;
    setState("games", (games) =>
      games.filter((g) => !(g.id === game.id && g.source === game.source)),
    );
  });

  const gameUninstallingUnlisten = listen<Game>(
    "game-uninstalling",
    (event) => {
      const game = event.payload;
      setState(
        "games",
        (g) => g.id === game.id && g.source === game.source,
        produce((g) => {
          g.status = "uninstalling";
        }),
      );
    },
  );

  const gameUninstalledUnlisten = listen<Game>("game-uninstalled", (event) => {
    const game = event.payload;
    setState(
      "games",
      (g) => g.id === game.id && g.source === game.source,
      produce((g) => {
        g.status = "notInstalled";
      }),
    );
  });

  const downloadQueuedUnlisten = listen<DownloadItem>(
    "download-queued",
    (event) => {
      const payload = event.payload;
      setState("downloadQueue", state.downloadQueue.length, payload);
      setState(
        "games",
        (g) => g.id === payload.gameId && g.source === payload.gameSource,
        produce((g) => {
          g.status = "downloading";
        }),
      );
    },
  );

  const downloadExternalUnlisten = listen<DownloadItem>(
    "download-external",
    (event) => {
      const payload = event.payload;
      setState("externalDownloads", state.externalDownloads.length, payload);
      setState(
        "games",
        (g) => g.id === payload.gameId && g.source === payload.gameSource,
        produce((g) => {
          g.status = "downloading";
        }),
      );
    },
  );

  const downloadProgressUnlisten = listen<DownloadItem>(
    "download-progress",
    (event) => {
      const payload = event.payload;
      setState(
        "downloadQueue",
        (i) =>
          i.gameId === payload.gameId && i.gameSource === payload.gameSource,
        produce((i) => {
          i.downloaded = payload.downloaded;
        }),
      );
    },
  );

  const downloadFinishedUnlisten = listen<DownloadItem>(
    "download-finished",
    (event) => {
      const payload = event.payload;
      setState(
        "downloadQueue",
        (i) =>
          i.gameId === payload.gameId && i.gameSource === payload.gameSource,
        produce((i) => {
          i.downloaded = payload.downloadSize;
        }),
      );
      setState(
        "externalDownloads",
        (i) =>
          i.gameId === payload.gameId && i.gameSource === payload.gameSource,
        produce((i) => {
          i.downloadSize = payload.downloadSize;
          i.downloaded = payload.downloadSize;
        }),
      );
    },
  );

  const downloadInstalledUnlisten = listen<DownloadItem>(
    "download-installed",
    (event) => {
      const payload = event.payload;
      setState("downloadQueue", (items) =>
        items.filter(
          (i) =>
            !(
              i.gameId === payload.gameId && i.gameSource === payload.gameSource
            ),
        ),
      );
      setState("externalDownloads", (items) =>
        items.filter(
          (i) =>
            !(
              i.gameId === payload.gameId && i.gameSource === payload.gameSource
            ),
        ),
      );
      setState("completedDownloads", state.completedDownloads.length, payload);

      setState(
        "games",
        (g) => g.id === payload.gameId && g.source === payload.gameSource,
        produce((g) => {
          g.status = "installed";
        }),
      );
    },
  );

  onCleanup(() => {
    gameHiddenUnlisten.then((u) => u());
    gameUninstallingUnlisten.then((u) => u());
    gameUninstalledUnlisten.then((u) => u());
    downloadQueuedUnlisten.then((u) => u());
    downloadExternalUnlisten.then((u) => u());
    downloadProgressUnlisten.then((u) => u());
    downloadFinishedUnlisten.then((u) => u());
    downloadInstalledUnlisten.then((u) => u());
  });

  return (
    <GameContext.Provider value={{ state, setState }}>
      {props.children}
    </GameContext.Provider>
  );
};

export default ContextProvider;
