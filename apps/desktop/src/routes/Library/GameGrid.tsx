import { createEffect, createSignal, For } from "solid-js";
import { groupArrayElements } from "../../util/array";
import { WindowEventListener } from "@solid-primitives/event-listener";
import { Virtualizer } from "virtua/solid";
import GameCard from "../../components/GameCard";
import { type Game } from "../../models/types";

interface GameGridProps {
  games: Game[];
  onGameClick: (game: Game) => void;
}

const GameGrid = (props: GameGridProps) => {
  const [columns, setColumns] = createSignal(4);

  createEffect(() => {
    calculateColumns();
  });

  let gameContainerRef: HTMLDivElement;

  function calculateColumns() {
    let numColumns = Math.floor(gameContainerRef.clientWidth / (192 + 36));
    if (numColumns < 1) numColumns = 1;
    setColumns(numColumns);
  }

  return (
    <>
      <WindowEventListener onResize={calculateColumns} />
      <div
        ref={(el) => (gameContainerRef = el)}
        class="mr-[14px] overflow-y-auto overflow-x-hidden pb-16"
        style={{ "scrollbar-gutter": "stable" }}
      >
        <Virtualizer
          data={groupArrayElements(props.games, columns())}
          overscan={1}
        >
          {(gameRow) => (
            <div class="my-24 ml-40 flex justify-between gap-24 pr-[20px]">
              <For each={gameRow}>
                {(game, i) => (
                  <>
                    <GameCard
                      game={game}
                      onClick={() => props.onGameClick(game)}
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
      </div>
    </>
  );
};

export default GameGrid;
