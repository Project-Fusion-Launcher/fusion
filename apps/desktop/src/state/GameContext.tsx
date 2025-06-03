import type { JSXElement } from "solid-js";
import { createContext, createEffect, onCleanup } from "solid-js";
import type { DownloadItem } from "../models/types";
import type { SetStoreFunction } from "solid-js/store";
import { createStore, produce } from "solid-js/store";
import { commands, events, type Game, type GameFilters } from "../bindings";

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

  function getGames(refetch = false, filters: GameFilters | null = null) {
    commands.getGames(refetch, filters).then((games) => {
      if (games.status === "error") {
        console.error("Failed to fetch games:", games.error);
        return;
      }
      setState("games", games.data);
    });
  }

  const gameHiddenUnlisten = events.gameHidden.listen((e) => {
    const payload = e.payload;
    setState("games", (games) =>
      games.filter(
        (g) => !(g.id === payload.gameId && g.source === payload.gameSource),
      ),
    );
  });

  const gameUninstallingUnlisten = events.gameUninstalling.listen((e) => {
    const payload = e.payload;
    setState(
      "games",
      (g) => g.id === payload.gameId && g.source === payload.gameSource,
      produce((g) => {
        g.status = "uninstalling";
      }),
    );
  });

  const gameUninstalledUnlisten = events.gameUninstalled.listen((e) => {
    const payload = e.payload;
    setState(
      "games",
      (g) => g.id === payload.gameId && g.source === payload.gameSource,
      produce((g) => {
        g.status = "notInstalled";
      }),
    );
  });

  const gameDownloadQueuedUnlisten = events.gameDownloadQueued.listen((e) => {
    const payload = e.payload;
    setState("downloadQueue", state.downloadQueue.length, {
      gameId: payload.gameId,
      gameSource: payload.gameSource,
      gameTitle: payload.gameTitle,
      downloadSize: payload.downloadSize,
      downloaded: payload.downloaded,
    });
    setState(
      "games",
      (g) => g.id === payload.gameId && g.source === payload.gameSource,
      produce((g) => {
        g.status = "downloading";
      }),
    );
  });

  const gameDownloadProgressUnlisten = events.gameDownloadProgress.listen(
    (e) => {
      const payload = e.payload;
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

  const gameInstallingUnlisten = events.gameInstalling.listen((e) => {
    const payload = e.payload;
    setState(
      "downloadQueue",
      (i) => i.gameId === payload.gameId && i.gameSource === payload.gameSource,
      produce((i) => {
        i.downloaded = i.downloadSize;
      }),
    );
    /*setState(
        "externalDownloads",
        (i) =>
          i.gameId === payload.gameId && i.gameSource === payload.gameSource,
        produce((i) => {
          i.downloadSize = payload.downloadSize;
          i.downloaded = payload.downloadSize;
        }),
      );*/
  });

  const gameInstalledUnlisten = events.gameInstalled.listen((e) => {
    const payload = e.payload;
    setState("downloadQueue", (items) =>
      items.filter((i) => {
        if (
          i.gameId === payload.gameId &&
          i.gameSource === payload.gameSource
        ) {
          setState("completedDownloads", state.completedDownloads.length, i);
          return false;
        }
        return true;
      }),
    );
    /*setState("externalDownloads", (items) =>
      items.filter(
        (i) =>
          !(i.gameId === payload.gameId && i.gameSource === payload.gameSource),
      ),
    );*/

    setState(
      "games",
      (g) => g.id === payload.gameId && g.source === payload.gameSource,
      produce((g) => {
        g.status = "installed";
      }),
    );
  });

  /*const downloadExternalUnlisten = listen<DownloadItem>(
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
  );*/

  onCleanup(() => {
    gameHiddenUnlisten.then((u) => u());
    gameUninstallingUnlisten.then((u) => u());
    gameUninstalledUnlisten.then((u) => u());
    gameDownloadQueuedUnlisten.then((u) => u());
    gameDownloadProgressUnlisten.then((u) => u());
    gameInstallingUnlisten.then((u) => u());
    gameInstalledUnlisten.then((u) => u());
    //downloadExternalUnlisten.then((u) => u());
  });

  return (
    <GameContext.Provider value={{ state, setState }}>
      {props.children}
    </GameContext.Provider>
  );
};

export default ContextProvider;
