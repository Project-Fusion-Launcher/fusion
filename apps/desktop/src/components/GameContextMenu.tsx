import { ContextMenu } from "@kobalte/core/context-menu";
import type { JSXElement } from "solid-js";
import { Match, Show, Switch, useContext } from "solid-js";
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
import { AppContext } from "../State";

interface GameContextMenuProps {
  game: Game | null;
  children: JSXElement;
  onMainAction?: () => void;
}

const GameContextMenu = (props: GameContextMenuProps) => {
  const { state } = useContext(AppContext);

  function handleUninstall() {
    if (props.game === null) return;
    state.uninstallGame(props.game);
  }

  function handleHide() {
    if (props.game === null) return;
    state.hideGame(props.game);
  }

  return (
    <ContextMenu>
      <ContextMenu.Trigger>{props.children}</ContextMenu.Trigger>
      <ContextMenu.Portal>
        <Show when={props.game !== null}>
          <ContextMenuContent>
            <Switch>
              <Match when={props.game?.status === "installed"}>
                <ContextMenuItem variant="accent" onSelect={props.onMainAction}>
                  <Play class="size-16" />
                  PLAY
                </ContextMenuItem>
              </Match>
              <Match when={props.game?.status === "notInstalled"}>
                <ContextMenuItem
                  variant="primary"
                  onSelect={props.onMainAction}
                >
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
                  <Switch>
                    <Match when={props.game?.hidden}>
                      <ContextMenuItem>
                        <Eye class="size-16" />
                        Unhide game
                      </ContextMenuItem>
                    </Match>
                    <Match when={!props.game?.hidden}>
                      <ContextMenuItem onSelect={handleHide}>
                        <EyeOff class="size-16" />
                        Hide game
                      </ContextMenuItem>
                    </Match>
                  </Switch>
                  <Show when={props.game?.status === "installed"}>
                    <ContextMenuItem>
                      <Folder class="size-16" />
                      Open install folder
                    </ContextMenuItem>
                    <ContextMenuItem
                      variant="danger"
                      onSelect={handleUninstall}
                    >
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
