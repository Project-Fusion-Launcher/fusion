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
import {
  type GameFilters,
  type GameFiltersStatus,
  type Game,
} from "../../models/types";
import GameGrid from "./GameGrid";
import { AppContext } from "../../State";
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

  const [currentGameStatus, setCurrentGameStatus] =
    createSignal<GameFiltersStatus>("all");

  const [filters, setFilters] = createStore<GameFilters>({
    query: "",
  });

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
    state.getGames(refetch, filters);
  }

  // Handle the main action button click event (launch or install)
  function handleMainAction(game: Game) {
    if (game.status === "installed") {
      invoke("launch_game", { gameId: game.id, gameSource: game.source });
    } else {
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
    setFilters("query", query);
    getGames(false);
  }

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
            selectedStatus={currentGameStatus()}
            status="all"
            onClick={setCurrentGameStatus}
            number={state.total}
          />
          <StatusFilterButton
            selectedStatus={currentGameStatus()}
            status="installed"
            onClick={setCurrentGameStatus}
            number={state.installed}
          />
          <StatusFilterButton
            selectedStatus={currentGameStatus()}
            status="notInstalled"
            onClick={setCurrentGameStatus}
            number={state.total - state.installed}
          />
          <Button variant="outline" onClick={() => getGames(true)}>
            <RefreshCcw class="text-primary" />
          </Button>
        </div>
      </div>
      <GameGrid
        games={state.games.filter(
          (game) =>
            game.status === currentGameStatus() ||
            currentGameStatus() === "all",
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
