import { createEffect, createSignal, For } from "solid-js";
import { groupArrayElements } from "../../utils/array";
import { WindowEventListener } from "@solid-primitives/event-listener";
import { Virtualizer } from "virtua/solid";
import GameCard from "../../components/GameCard";
import { type Game } from "../../models/types";
import GameContextMenu from "../../components/GameContextMenu";

interface GameGridProps {
  games: Game[];
  onMainAction: (game: Game) => void;
}

const GameGrid = (props: GameGridProps) => {
  const [columns, setColumns] = createSignal(4);
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);

  let gameContainerRef: HTMLDivElement;

  // Calculate the initial number of columns
  createEffect(() => {
    calculateColumns();
  });

  // Calculate the number of columns based on the container width
  function calculateColumns() {
    // TODO: allow for custom game sizes
    // 192px is the width of the game card
    // 24px is the gap between game cards
    // 16px is the left padding of the game container minus game gap
    // 20px is the right padding
    let numColumns = Math.floor(
      (gameContainerRef.clientWidth - 16 - 20) / (192 + 24),
    );
    if (numColumns < 1) numColumns = 1;
    setColumns(numColumns);
  }

  function handleMainAction(game: Game | null) {
    if (game) props.onMainAction(game);
  }

  return (
    <>
      <WindowEventListener onResize={calculateColumns} />
      <div
        ref={(el) => (gameContainerRef = el)}
        class="mr-[14px] overflow-y-auto overflow-x-hidden pb-16"
        style={{ "scrollbar-gutter": "stable" }}
      >
        {/* There cannot be a context menu per game as it causes too much scroll lag.
            There is probably a better way to handle this.
        */}
        <GameContextMenu
          game={selectedGame()}
          onMainAction={() => handleMainAction(selectedGame())}
        >
          <Virtualizer
            data={groupArrayElements(props.games, columns())}
            overscan={1}
          >
            {(gameRow) => (
              <div
                class="game-row flex justify-between gap-24 py-24 pl-40 pr-[20px]"
                onContextMenu={(e) => {
                  if (e.target.classList.contains("game-row")) {
                    setSelectedGame(null);
                  }
                }}
              >
                <For each={gameRow}>
                  {(game, i) => (
                    <>
                      <GameCard
                        game={game}
                        onClick={() => handleMainAction(game)}
                        onContextMenu={() => setSelectedGame(game)}
                      />
                      {/* Fill empty spots in the last row with divs */}
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
        </GameContextMenu>
      </div>
    </>
  );
};

export default GameGrid;
