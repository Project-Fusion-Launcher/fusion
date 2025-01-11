import type { JSX } from "solid-js";
import { createContext, onCleanup } from "solid-js";
import type { DownloadItem, Game, GameFilters } from "./models/types";
import type { SetStoreFunction } from "solid-js/store";
import { createStore, produce } from "solid-js/store";
import { listen } from "@tauri-apps/api/event";
import type { GameUninstalledPayload } from "./models/payloads";
import { invoke } from "@tauri-apps/api/core";

export const AppContext = createContext<{
  state: {
    games: Game[];
    downloadQueue: DownloadItem[];
    externalDownloads: DownloadItem[];
    completedDownloads: DownloadItem[];
    total: number;
    installed: number;
    hideGame: (game: Game) => void;
    getGames: (refetch?: boolean, filters?: GameFilters) => void;
    uninstallGame: (game: Game) => void;
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
    hideGame: () => {},
    getGames: () => {},
    uninstallGame: () => {},
  },
  setState: () => {},
});

interface StateProps {
  children: JSX.Element;
}

const ContextProvider = (props: StateProps) => {
  const [state, setState] = createStore({
    games: [] as Game[],
    downloadQueue: [] as DownloadItem[],
    externalDownloads: [] as DownloadItem[],
    completedDownloads: [] as DownloadItem[],
    total: 0,
    installed: 0,
    hideGame,
    getGames,
    uninstallGame,
  });

  // Helper function to update the game count of currently active games
  function refreshGameCount() {
    setState("total", state.games.length);
    setState(
      "installed",
      state.games.filter((g) => g.status === "installed").length,
    );
  }

  /* INVOKERS */
  function getGames(refetch = false, filters?: GameFilters) {
    invoke<Game[]>("get_games", { refetch, filters }).then((games) => {
      setState("games", games);
      refreshGameCount();
    });
  }

  function hideGame(game: Game) {
    invoke<void>("hide_game", {
      gameId: game.id,
      gameSource: game.source,
    }).then(() => {
      setState("games", (games) =>
        games.filter((g) => !(g.id === game.id && g.source === game.source)),
      );
      refreshGameCount();
    });
  }

  function uninstallGame(game: Game) {
    setState(
      "games",
      (g) => g.id === game.id && g.source === game.source,
      produce((g) => {
        g.status = "uninstalling";
      }),
    );

    invoke<void>("uninstall_game", {
      gameId: game.id,
      gameSource: game.source,
    }).then(() => {});
  }

  /* LISTENERS */
  const downloadQueuedUnlisten = listen<DownloadItem>(
    "download-queued",
    (event) => {
      const payload = event.payload;
      setState("downloadQueue", state.downloadQueue.length, payload);
    },
  );

  const downloadExternalUnlisten = listen<DownloadItem>(
    "download-external",
    (event) => {
      const payload = event.payload;
      setState("externalDownloads", state.externalDownloads.length, payload);
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
          console.log("found");
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
      refreshGameCount();
    },
  );

  const gameUninstalledUnlisten = listen<GameUninstalledPayload>(
    "game-uninstalled",
    (event) => {
      const payload = event.payload;
      setState(
        "games",
        (game) =>
          game.id === payload.gameId && game.source === payload.gameSource,
        produce((game) => {
          game.status = "notInstalled";
        }),
      );
      refreshGameCount();
    },
  );

  onCleanup(() => {
    // This component should never unmount, but unlisten just in case
    downloadQueuedUnlisten.then((u) => u());
    downloadExternalUnlisten.then((u) => u());
    downloadProgressUnlisten.then((u) => u());
    downloadFinishedUnlisten.then((u) => u());
    downloadInstalledUnlisten.then((u) => u());
    gameUninstalledUnlisten.then((u) => u());
  });

  return (
    <AppContext.Provider value={{ state, setState }}>
      {props.children}
    </AppContext.Provider>
  );
};

export default ContextProvider;
