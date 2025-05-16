import type { JSXElement } from "solid-js";
import { Match, Show, Switch } from "solid-js";
import type { Game } from "../models/types";
import {
  ArrowDownToLine,
  Eye,
  EyeOff,
  Folder,
  Pen,
  Play,
  Plus,
  RefreshCcw,
  Star,
  StarOff,
  Trash2,
  Wrench,
} from "lucide-solid";
import {
  ContextMenu,
  ContextMenuTrigger,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSub,
  ContextMenuSubTrigger,
  ContextMenuSubContent,
  ContextMenuSeparator,
} from "@repo/ui";
import { openPath } from "@tauri-apps/plugin-opener";
import { hideGame, uninstallGame } from "../services/game";

interface GameContextMenuProps {
  game: Game | null;
  children: JSXElement;
  onMainAction?: () => void;
}

const GameContextMenu = (props: GameContextMenuProps) => {
  function handleUninstall() {
    if (props.game === null) return;
    uninstallGame(props.game);
  }

  function handleHide() {
    if (props.game === null) return;
    hideGame(props.game);
  }

  return (
    <ContextMenu>
      <ContextMenuTrigger class="relative h-full overflow-hidden">
        {props.children}
      </ContextMenuTrigger>
      <ContextMenuContent>
        <Switch>
          <Match when={props.game}>
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
            <ContextMenuItem disabled>
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
            <ContextMenuSub gutter={4}>
              <ContextMenuSubTrigger>
                <Wrench class="size-16" />
                Manage
              </ContextMenuSubTrigger>
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
                  <ContextMenuItem
                    onSelect={() => openPath(props.game?.path || "")}
                  >
                    <Folder class="size-16" />
                    Open install folder
                  </ContextMenuItem>
                  <ContextMenuItem
                    variant="destructive"
                    onSelect={handleUninstall}
                  >
                    <Trash2 class="size-16" />
                    Uninstall
                  </ContextMenuItem>
                </Show>
              </ContextMenuSubContent>
            </ContextMenuSub>
            <ContextMenuSeparator />
            <ContextMenuItem disabled>
              <Pen class="size-16" />
              Properties
            </ContextMenuItem>
          </Match>
          <Match when={!props.game}>
            <ContextMenuItem disabled>
              <Plus class="size-16" />
              Add game
            </ContextMenuItem>
            <ContextMenuItem disabled>
              <RefreshCcw class="size-16" />
              Refresh games
            </ContextMenuItem>
          </Match>
        </Switch>
      </ContextMenuContent>
    </ContextMenu>
  );
};

export default GameContextMenu;
