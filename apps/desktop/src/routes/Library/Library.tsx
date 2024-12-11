import { Badge, Button } from "@repo/ui";
import Header from "../../components/Header";
import {
  createRenderEffect,
  createSignal,
  Match,
  onCleanup,
  Switch,
  useContext,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCcw } from "lucide-solid";
import InstallDialog from "../../components/InstallDialog";
import { type Game } from "../../models/types";
import GameGrid from "./GameGrid";
import { AppContext } from "../../State";
import type {
  GameFiltersPayload,
  GameFiltersStatus,
} from "../../models/payloads";
import { createStore } from "solid-js/store";

interface StatusFilterButtonProps {
  selectedStatus?: GameFiltersStatus;
  status: GameFiltersStatus;
  number: number;
  onClick: (status: GameFiltersStatus) => void;
}

const StatusFilterButton = (props: StatusFilterButtonProps) => {
  return (
    <Button variant="ghost" onClick={() => props.onClick(props.status)}>
      <span
        class="whitespace-nowrap transition-all"
        classList={{
          "text-primary": props.selectedStatus === props.status,
          "text-secondary": props.selectedStatus !== props.status,
        }}
      >
        <Switch>
          <Match when={props.status === "all"}>All Games</Match>
          <Match when={props.status === "installed"}>Installed</Match>
          <Match when={props.status === "notInstalled"}>Not Installed</Match>
        </Switch>
      </span>
      <Badge
        variant={props.selectedStatus === props.status ? "accent" : "outline"}
      >
        {props.number}
      </Badge>
    </Button>
  );
};

const Library = () => {
  const { state, setState } = useContext(AppContext);

  const [isDialogOpen, setIsDialogOpen] = createSignal(false);
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);

  const [filters, setFilters] = createStore<GameFiltersPayload>({
    query: "",
    status: "all",
  });

  createRenderEffect(() => {
    getGames(false);
  });

  function handleNewGames(newGames: Game[]) {
    setState("games", newGames);
    if (filters.status === "all") {
      setState("total", newGames.length);
      setState(
        "installed",
        newGames.filter((g) => g.status === "installed").length,
      );
    }
  }

  function getGames(refetch: boolean) {
    invoke<Game[]>("get_games", { refetch, filters: filters }).then(
      handleNewGames,
    );
  }

  function handleGameClick(game: Game) {
    if (game.status === "installed") {
      invoke("launch_game", { gameId: game.id, gameSource: game.source });
    } else {
      setSelectedGame(game);
      setIsDialogOpen(true);
    }
  }

  function handleDialogClose() {
    setIsDialogOpen(false);
    setTimeout(() => {
      setSelectedGame(null);
    }, 300);
  }

  function handleQueryChange(query: string) {
    setFilters("query", query);
    getGames(false);
  }

  function handleStatusChange(status: GameFiltersStatus) {
    setFilters("status", status);
    getGames(false);
  }

  onCleanup(() => {
    setState("games", []);
  });

  return (
    <>
      <Header
        title="Library"
        query={filters.query}
        setQuery={handleQueryChange}
      />
      <div class="mb-16 h-28 px-40">
        <div class="flex h-full w-min items-center gap-40">
          <StatusFilterButton
            selectedStatus={filters.status}
            status="all"
            onClick={handleStatusChange}
            number={state.total}
          />
          <StatusFilterButton
            selectedStatus={filters.status}
            status="installed"
            onClick={handleStatusChange}
            number={state.installed}
          />
          <StatusFilterButton
            selectedStatus={filters.status}
            status="notInstalled"
            onClick={handleStatusChange}
            number={state.total - state.installed}
          />
          <Button variant="outline" onClick={() => getGames(true)}>
            <RefreshCcw class="text-primary" />
          </Button>
        </div>
      </div>
      <GameGrid games={state.games} onGameClick={handleGameClick} />
      <InstallDialog
        selectedGame={selectedGame()}
        open={isDialogOpen()}
        handleDialogClose={handleDialogClose}
      />
    </>
  );
};

export default Library;
