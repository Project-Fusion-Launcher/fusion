import { Button } from "@kobalte/core/button";
import { ArrowDownToLine, Play } from "lucide-solid";
import { Match, Switch } from "solid-js";
import { type Game } from "../models/types";

interface GameCardProps {
  game: Game;
  onClick: () => void;
  onContextMenu: () => void;
}

const GameCard = (props: GameCardProps) => {
  return (
    <Button
      class="w-192 group flex flex-shrink-0 flex-col hover:cursor-pointer"
      onClick={() => props.onClick()}
      onContextMenu={() => props.onContextMenu()}
    >
      <div class="bg-secondary h-288 group-hover:outline-accent relative w-full overflow-hidden rounded outline-none outline-2 transition-all group-hover:shadow-[0_0_1.5rem_rgba(255,255,255,0.25)]">
        <img
          class="absolute h-full object-cover"
          src={props.game.coverUrl}
          loading="lazy"
        />
        <div class="bg-background absolute z-10 flex h-full w-full items-center justify-center bg-opacity-60 opacity-0 backdrop-blur-sm transition-opacity group-hover:opacity-100">
          <Switch>
            <Match when={props.game.status === "installed"}>
              <Play class="stroke-primary size-1/3" />
            </Match>
            <Match when={props.game.status === "notInstalled"}>
              <ArrowDownToLine class="stroke-primary size-1/3" />
            </Match>
          </Switch>
        </div>
      </div>
      <span class="text-primary mb-8 mt-16 w-full overflow-hidden text-ellipsis text-nowrap text-left text-base font-medium">
        {props.game.title}
      </span>
      <span class="text-secondary w-full overflow-hidden text-ellipsis text-nowrap text-left text-[14px] font-medium leading-[14px]">
        {props.game.developer}
      </span>
    </Button>
  );
};

export default GameCard;
