import { Badge, Button, Dialog, Select } from "@repo/ui";
import Header from "../components/Header";
import GameCard from "../components/GameCard";
import { createEffect, createResource, createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCcw } from "lucide-solid";
import { Virtualizer } from "virtua/solid";
import { groupArrayElements } from "../util/array";

interface Game {
  id: string;
  title: string;
  source: string;
  developer?: string;
}

const Library = () => {
  const [games, { refetch }] = createResource(fetchGames);
  const [columns, setColumns] = createSignal(4);

  const [isDialogOpen, setIsDialogOpen] = createSignal(false);
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);

  let gameContainerRef: HTMLDivElement;

  async function fetchGames(
    _source: unknown,
    { refetching }: { refetching: boolean },
  ): Promise<Game[]> {
    const newGames = (await invoke("get_games", {
      refetch: refetching,
    }).catch(() => [])) as Game[];
    newGames.sort((a, b) => a.title.localeCompare(b.title));
    return newGames;
  }

  createEffect(() => {
    const calculateColumns = () => {
      let numColumns = Math.floor(gameContainerRef.clientWidth / (192 + 36));
      if (numColumns < 1) numColumns = 1;
      setColumns(numColumns);
    };

    window.addEventListener("resize", () => calculateColumns());

    calculateColumns();
  });

  const handleGameClick = (game: Game) => {
    setSelectedGame(game);
    setIsDialogOpen(true);
  };

  const handleDialogClose = () => {
    setIsDialogOpen(false);
    setTimeout(() => setSelectedGame(null), 300);
  };

  return (
    <>
      <Header title="Library" />
      <div class="h-28 px-40">
        <div class="flex h-full w-min items-center gap-40">
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">All Games</span>
            <Badge variant="accent">100</Badge>
          </Button>
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">Installed</span>
            <Badge variant="outline">69</Badge>
          </Button>
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">Not Installed</span>
            <Badge variant="outline">31</Badge>
          </Button>
          <Button variant="outline" onClick={() => refetch()}>
            <RefreshCcw class="text-primary" />
          </Button>
        </div>
      </div>
      <div
        ref={(el) => (gameContainerRef = el)}
        class="mr-[14px] overflow-y-auto overflow-x-hidden pb-40"
        style={{ "scrollbar-gutter": "stable" }}
      >
        <Virtualizer data={groupArrayElements(games(), columns())} overscan={1}>
          {(gameRow) => (
            <div class="ml-40 mt-40 flex justify-between gap-24 pr-[20px]">
              <For each={gameRow}>
                {(game, i) => (
                  <>
                    <GameCard
                      title={game.title}
                      developer={game.developer}
                      onClick={() => handleGameClick(game)}
                    />
                    {i() === gameRow.length - 1 && i() < columns() - 1 && (
                      <For
                        each={Array.from({
                          length: columns() - gameRow.length,
                        })}
                      >
                        {() => <div class="w-192 h-228" />}
                      </For>
                    )}
                  </>
                )}
              </For>
            </div>
          )}
        </Virtualizer>
      </div>
      <Dialog
        title={
          "Install" + (selectedGame() ? ` ${selectedGame()?.title}` : "Game")
        }
        defaultOpen
        open={isDialogOpen()}
        onOpenChange={handleDialogClose}
      >
        <Select
          variant="outline"
          placeholder="Choose a version"
          options={["Option 1", "Option 2", "Option 3"]}
        />
      </Dialog>
    </>
  );
};

export default Library;
