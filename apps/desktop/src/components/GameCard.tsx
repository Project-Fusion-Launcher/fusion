import { Button } from "@kobalte/core/button";
import { ArrowDownToLine, LoaderCircle, Play } from "lucide-solid";
import { Match, Switch } from "solid-js";
import { type Game } from "../models/types";
import { Tooltip } from "@repo/ui";

interface GameCardProps {
  game: Game;
  onClick: () => void;
  onContextMenu: () => void;
}

const GameCard = (props: GameCardProps) => {
  return (
    <Button
      class="w-192 group flex shrink-0 flex-col hover:cursor-pointer"
      onClick={props.onClick}
      onContextMenu={props.onContextMenu}
    >
      <div class="bg-secondary h-288 group-hover:outline-accent relative w-full overflow-hidden rounded-md outline-2 outline-transparent transition-all group-hover:shadow-[0_0_1rem_rgba(255,255,255,0.25)]">
        <img
          class="absolute h-full object-cover"
          src={props.game.coverUrl}
          loading="lazy"
        />
        <div
          class={`bg-background/60 backdrop-blur-xs absolute z-10 flex h-full w-full items-center justify-center transition-opacity group-hover:opacity-100 ${props.game.status === "uninstalling" ? "opacity-100" : "opacity-0"}`}
        >
          <div
            class={`bg-accent drop-shadow-xs absolute size-72 rounded-full p-20 transition-all group-hover:translate-y-0 ${props.game.status === "uninstalling" ? "translate-y-0" : "translate-y-[200%]"}`}
          >
            <Switch>
              <Match
                when={
                  props.game.status === "uninstalling" ||
                  props.game.status === "installing" ||
                  props.game.status === "downloading"
                }
              >
                <LoaderCircle class="stroke-primary size-full animate-spin" />
              </Match>
              <Match when={props.game.status === "installed"}>
                <Play class="stroke-primary size-full" />
              </Match>
              <Match when={props.game.status === "notInstalled"}>
                <ArrowDownToLine class="stroke-primary size-full" />
              </Match>
            </Switch>
          </div>
        </div>
      </div>
      <Tooltip
        variant="outline"
        content={props.game.title}
        as="span"
        class="text-primary mb-8 mt-16 w-full overflow-hidden text-ellipsis text-nowrap text-left text-base font-medium"
      >
        {props.game.title}
      </Tooltip>
      <span class="text-secondary w-full overflow-hidden text-ellipsis text-nowrap text-left text-[14px] font-medium leading-[14px]">
        {props.game.developer}
      </span>
    </Button>
  );
};

export default GameCard;
