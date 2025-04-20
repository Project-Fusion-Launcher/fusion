import { Badge, Button, Tabs, TabsList, TabsTrigger } from "@repo/ui";
import Header from "../../components/Header";
import {
  createMemo,
  createRenderEffect,
  createSignal,
  onCleanup,
  useContext,
} from "solid-js";
import { RefreshCcw } from "lucide-solid";
import InstallDialog from "../../components/InstallDialog";
import { type GameFiltersStatus, type Game } from "../../models/types";
import GameGrid from "./GameGrid";
import { useSearchParams } from "@solidjs/router";
import { parseSearchParam } from "../../utils/string";
import { GameContext } from "../../state/GameContext";
import { launchGame } from "../../services/game";

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
  const { state, setState } = useContext(GameContext);
  const [searchParams, setSearchParams] = useSearchParams();

  const [isDialogOpen, setIsDialogOpen] = createSignal(false);
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);

  const selectedGameStatus = createMemo<GameFiltersStatus>(
    () => parseSearchParam(searchParams.status) || "all",
  );

  const query = createMemo<string | undefined>(() =>
    parseSearchParam(searchParams.query),
  );

  createRenderEffect(() => {
    getGames(false);
  });

  onCleanup(() => {
    setState("games", []);
  });

  function getGames(refetch: boolean) {
    state.getGames(refetch, { query: query() });
  }

  function handleMainAction(game: Game) {
    if (game.status === "installed") {
      launchGame(game);
    } else if (game.status === "notInstalled") {
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
    setSearchParams({ query });
  }

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
        <Tabs onChange={handleGameStatusChange} value={selectedGameStatus()}>
          <TabsList>
            <TabsTrigger value="all">
              <StatusFilterButton
                status={selectedGameStatus()}
                value="all"
                name="All games"
                number={state.total}
              />
            </TabsTrigger>
            <TabsTrigger value="installed">
              <StatusFilterButton
                status={selectedGameStatus()}
                value="installed"
                name="Installed"
                number={state.installed}
              />
            </TabsTrigger>
            <TabsTrigger value="notInstalled">
              <StatusFilterButton
                status={selectedGameStatus()}
                value="notInstalled"
                name="Not Installed"
                number={state.total - state.installed}
              />
            </TabsTrigger>
          </TabsList>
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
