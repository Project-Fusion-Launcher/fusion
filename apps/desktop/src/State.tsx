import type { JSX } from "solid-js";
import { createContext, onCleanup } from "solid-js";
import type { Game } from "./models/types";
import type { SetStoreFunction } from "solid-js/store";
import { createStore, produce } from "solid-js/store";
import { listen } from "@tauri-apps/api/event";
import type {
  DownloadFinishedPayload,
  GameUninstalledPayload,
} from "./models/payloads";

export const AppContext = createContext<{
  state: {
    games: Game[];
    total: number;
    installed: number;
  };
  setState: SetStoreFunction<{
    games: Game[];
    total: number;
    installed: number;
  }>;
}>({
  state: {
    games: [],
    total: 0,
    installed: 0,
  },
  setState: () => {},
});

interface StateProps {
  children: JSX.Element;
}

const ContextProvider = (props: StateProps) => {
  const [state, setState] = createStore({
    games: [] as Game[],
    total: 0,
    installed: 0,
  });

  /* INVOKERS */

  /* LISTENERS */
  const downloadFinishedUnlisten = listen<DownloadFinishedPayload>(
    "download-finished",
    (event) => {
      const payload = event.payload;
      setState(
        "games",
        (game) =>
          game.id === payload.gameId && game.source === payload.gameSource,
        produce((game) => {
          game.status = "installed";
        }),
      );
      setState("installed", (installed) => installed + 1);
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
      setState("installed", (installed) => installed - 1);
    },
  );

  onCleanup(() => {
    // This component should never unmount, but unlisten just in case
    downloadFinishedUnlisten.then((u) => u());
    gameUninstalledUnlisten.then((u) => u());
  });

  return (
    <AppContext.Provider value={{ state, setState }}>
      {props.children}
    </AppContext.Provider>
  );
};

export default ContextProvider;
