import { Badge, Button } from "@repo/ui";
import Header from "../../components/Header";
import {
  createRenderEffect,
  createSignal,
  onCleanup,
  useContext,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCcw } from "lucide-solid";
import InstallDialog from "../../components/InstallDialog";
import { type Game } from "../../models/types";
import GameGrid from "./GameGrid";
import { AppContext } from "../../State";

const Library = () => {
  const { state, setState } = useContext(AppContext);

  const [isDialogOpen, setIsDialogOpen] = createSignal(false);
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);

  createRenderEffect(() => {
    getGames(false);
  });

  function getGames(refetch: boolean) {
    invoke<Game[]>("get_games", { refetch }).then((newGames) => {
      newGames.sort((a, b) => a.title.localeCompare(b.title));
      setState("games", newGames);
      setState("total", newGames.length);
      setState(
        "installed",
        newGames.filter((g) => g.status === "installed").length,
      );
    });
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

  onCleanup(() => {
    setState("games", []);
  });

  return (
    <>
      <Header title="Library" />
      <div class="mb-16 h-28 px-40">
        <div class="flex h-full w-min items-center gap-40">
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">All Games</span>
            <Badge variant="accent">{state.total}</Badge>
          </Button>
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">Installed</span>
            <Badge variant="outline">{state.installed}</Badge>
          </Button>
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">Not Installed</span>
            <Badge variant="outline">{state.total - state.installed}</Badge>
          </Button>
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
