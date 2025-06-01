import { Button } from "@kobalte/core/button";
import { ArrowDownToLine, LoaderCircle, Play } from "lucide-solid";
import { Match, Switch } from "solid-js";
import { Tooltip, TooltipContent, TooltipTrigger } from "@repo/ui";
import type { Game } from "../bindings";

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
      <div class="bg-secondary h-288 group-hover:outline-accent relative w-full overflow-hidden rounded-md outline-2 outline-offset-4 outline-transparent transition-all group-hover:shadow-[0_0_1.2rem_rgba(255,255,255,0.25)]">
        <img
          class="absolute h-full object-cover"
          src={props.game.coverUrl ?? undefined}
          loading="lazy"
        />
        <div
          class={`bg-background/60 absolute z-10 flex h-full w-full items-center justify-center backdrop-blur-sm transition-opacity group-hover:opacity-100 ${props.game.status === "uninstalling" ? "opacity-100" : "opacity-0"}`}
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
      <Tooltip>
        <TooltipTrigger
          as="span"
          class="mb-8 mt-16 w-full overflow-hidden text-ellipsis text-nowrap text-left font-bold"
        >
          {props.game.title}
        </TooltipTrigger>
        <TooltipContent>{props.game.title}</TooltipContent>
      </Tooltip>
      <span class="text-secondary w-full overflow-hidden text-ellipsis text-nowrap text-left text-sm">
        {props.game.developer}
      </span>
    </Button>
  );
};

export default GameCard;
