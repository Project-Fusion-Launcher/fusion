import { Badge, Button, Tabs } from "@repo/ui";
import Header from "../../components/Header";
import {
  createMemo,
  createRenderEffect,
  createSignal,
  onCleanup,
  useContext,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCcw } from "lucide-solid";
import InstallDialog from "../../components/InstallDialog";
import { type GameFiltersStatus, type Game } from "../../models/types";
import GameGrid from "./GameGrid";
import { AppContext } from "../../State";
import { useSearchParams } from "@solidjs/router";
import { parseSearchParam } from "../../util/string";

interface StatusFilterButtonProps {
  status: GameFiltersStatus;
  value: string;
  name: string;
  number: number;
}

const StatusFilterButton = (props: StatusFilterButtonProps) => {
  return (
    <span class="flex items-center gap-8 text-nowrap">
      {props.name}
      <Badge variant={props.status === props.value ? "primary" : "outline"}>
        {props.number}
      </Badge>
    </span>
  );
};

const Library = () => {
  const { state, setState } = useContext(AppContext);
  const [searchParams, setSearchParams] = useSearchParams();

  const [isDialogOpen, setIsDialogOpen] = createSignal(false);
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);

  const selectedGameStatus = createMemo<GameFiltersStatus>(
    () => parseSearchParam(searchParams.status) || "all",
  );

  const query = createMemo<string | undefined>(() =>
    parseSearchParam(searchParams.query),
  );

  // Fetch the games on component mount
  createRenderEffect(() => {
    getGames(false);
  });

  // Clear the games on component unmount as there's no point
  // in keeping them in memory
  onCleanup(() => {
    setState("games", []);
  });

  // Fetch the games from the backend
  function getGames(refetch: boolean) {
    state.getGames(refetch, { query: query() });
  }

  // Handle the main action button click event (launch or install)
  function handleMainAction(game: Game) {
    if (game.status === "installed") {
      invoke("launch_game", { gameId: game.id, gameSource: game.source });
    } else if (game.status === "notInstalled") {
      setSelectedGame(game);
      setIsDialogOpen(true);
    }
  }

  // Handle the install dialog close event
  function handleDialogClose() {
    setIsDialogOpen(false);
    setTimeout(() => {
      setSelectedGame(null);
    }, 300);
  }

  // Handle the query change event
  function handleQueryChange(query: string) {
    setSearchParams({ query });
  }

  // Handle the game status change event
  function handleGameStatusChange(value: string) {
    const status = value as GameFiltersStatus;
    setSearchParams({ status });
  }

  return (
    <>
      <Header
        title="Library"
        query={query()}
        onQueryInput={handleQueryChange}
      />
      <div class="mb-16 flex h-28 items-center justify-between px-40">
        <Tabs
          values={["all", "installed", "notInstalled"]}
          onChange={handleGameStatusChange}
          value={selectedGameStatus()}
        >
          <StatusFilterButton
            status={selectedGameStatus()}
            value="all"
            name="All games"
            number={state.total}
          />
          <StatusFilterButton
            status={selectedGameStatus()}
            value="installed"
            name="Installed"
            number={state.installed}
          />
          <StatusFilterButton
            status={selectedGameStatus()}
            value="notInstalled"
            name="Not Installed"
            number={state.total - state.installed}
          />
        </Tabs>
        <Button variant="outline" size="sm" onClick={() => getGames(true)}>
          <RefreshCcw class="text-primary" />
        </Button>
      </div>
      <GameGrid
        games={state.games.filter(
          (game) =>
            game.status === selectedGameStatus() ||
            selectedGameStatus() === "all" ||
            game.status === "uninstalling",
        )}
        onMainAction={handleMainAction}
      />
      <InstallDialog
        selectedGame={selectedGame()}
        open={isDialogOpen()}
        onDialogClose={handleDialogClose}
      />
    </>
  );
};

export default Library;
