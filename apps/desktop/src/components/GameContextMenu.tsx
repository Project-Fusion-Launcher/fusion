import { ContextMenu } from "@kobalte/core/context-menu";
import { Match, Show, Switch, type JSX } from "solid-js";
import type { Game } from "../models/types";
import {
  ArrowDownToLine,
  ChevronRight,
  Eye,
  EyeOff,
  Folder,
  Pen,
  Play,
  Star,
  StarOff,
  Trash2,
  Wrench,
} from "lucide-solid";
import {
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
} from "@repo/ui";

interface GameContextMenuProps {
  game: Game | null;
  children: JSX.Element;
}

const GameContextMenu = (props: GameContextMenuProps) => {
  return (
    <ContextMenu>
      <ContextMenu.Trigger>{props.children}</ContextMenu.Trigger>
      <ContextMenu.Portal>
        <Show when={props.game !== null}>
          <ContextMenuContent>
            <Switch>
              <Match when={props.game?.status === "installed"}>
                <ContextMenuItem variant="accent">
                  <Play class="size-16" />
                  PLAY
                </ContextMenuItem>
              </Match>
              <Match when={props.game?.status === "notInstalled"}>
                <ContextMenuItem variant="primary">
                  <ArrowDownToLine class="size-16" />
                  INSTALL
                </ContextMenuItem>
              </Match>
            </Switch>
            <ContextMenuItem>
              <Switch>
                <Match when={props.game?.favorite}>
                  <StarOff class="size-16" />
                  Remove from favorites
                </Match>
                <Match when={!props.game?.favorite}>
                  <Star class="size-16" />
                  Add to favorites
                </Match>
              </Switch>
            </ContextMenuItem>
            <ContextMenu.Sub gutter={4}>
              <ContextMenuSubTrigger>
                <Wrench class="size-16" />
                Manage
                <ChevronRight class="size-16" />
              </ContextMenuSubTrigger>
              <ContextMenu.Portal>
                <ContextMenuSubContent>
                  <ContextMenuItem>
                    <Switch>
                      <Match when={props.game?.hidden}>
                        <Eye class="size-16" />
                        Unhide game
                      </Match>
                      <Match when={!props.game?.hidden}>
                        <EyeOff class="size-16" />
                        Hide game
                      </Match>
                    </Switch>
                  </ContextMenuItem>
                  <Show when={props.game?.status === "installed"}>
                    <ContextMenuItem>
                      <Folder class="size-16" />
                      Open install folder
                    </ContextMenuItem>
                    <ContextMenuItem variant="danger">
                      <Trash2 class="size-16" />
                      Uninstall
                    </ContextMenuItem>
                  </Show>
                </ContextMenuSubContent>
              </ContextMenu.Portal>
            </ContextMenu.Sub>
            <ContextMenuSeparator />
            <ContextMenuItem>
              <Pen class="size-16" />
              Properties
            </ContextMenuItem>
          </ContextMenuContent>
        </Show>
      </ContextMenu.Portal>
    </ContextMenu>
  );
};

export default GameContextMenu;