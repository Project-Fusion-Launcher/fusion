import type { JSX } from "solid-js";
import { For, useContext, Show, Switch, Match } from "solid-js";
import Header from "../../components/Header";
import DownloadItem from "./DownloadItem";
import { AppContext } from "../../State";
import type { GameSource } from "../../models/types";
import { TransitionGroup } from "solid-transition-group";
import { Info } from "lucide-solid";
import { Button } from "@repo/ui";

interface SectionProps {
  title: string;
  children: JSX.Element;
  count: number;
  onRemoveAll?: () => void;
}

const Section = (props: SectionProps) => {
  return (
    <div class="flex flex-col gap-40">
      <div class="flex h-24 items-center justify-between">
        <span
          class="text-primary opacity-1 text-lg font-medium transition-opacity"
          classList={{
            "opacity-0": props.title === "Completed" && props.count === 0,
          }}
        >
          {props.title} <span class="text-secondary">({props.count})</span>
        </span>
        <Show when={props.title != "Up Next" && props.count > 0}>
          <Button variant="ghost" onClick={props.onRemoveAll}>
            <span class="text-secondary font-medium">Remove all</span>
          </Button>
        </Show>
      </div>
      <div class="flex flex-col gap-16">{props.children}</div>
    </div>
  );
};

interface NoticeProps {
  children: string;
}

const Notice = (props: NoticeProps) => {
  return (
    <span class="text-secondary flex items-center gap-8 italic">
      <Info class="size-16" />
      {props.children}
    </span>
  );
};

const Downloads = () => {
  const { state, setState } = useContext(AppContext);

  // Handle removing a completed download
  function handleRemoveCompleted(gameId: string, gameSource: GameSource) {
    setState("downloadCompleted", (items) =>
      items.filter(
        (i) => !(i.gameId === gameId && i.gameSource === gameSource),
      ),
    );
  }

  // Handle removing all completed downloads
  function handleRemoveCompletedAll() {
    setState("downloadCompleted", []);
  }

  return (
    <>
      <Header title="Downloads" hideSearch />
      <div class="h-136 min-h-136 border-border text-primary w-full border-b">
        Details go here
      </div>
      <div class="h-full overflow-hidden pl-40 pr-[14px] pt-40">
        <div
          class="flex h-full flex-col gap-40 overflow-y-auto pr-20"
          style={{ "scrollbar-gutter": "stable" }}
        >
          <Section title="Up Next" count={state.downloadQueue.length}>
            <Switch>
              <Match when={state.downloadQueue.length === 0}>
                <Notice>There are no games in the queue.</Notice>
              </Match>
              <Match when={state.downloadQueue.length > 0}>
                <For each={state.downloadQueue}>
                  {(item) => <DownloadItem item={item} />}
                </For>
              </Match>
            </Switch>
          </Section>
          <Section
            title="Completed"
            count={state.downloadCompleted.length}
            onRemoveAll={handleRemoveCompletedAll}
          >
            <TransitionGroup
              onExit={(el, done) => {
                const a = el.animate(
                  [
                    { transform: "translate(0)", opacity: 1 },
                    { transform: "translate(-100%)", opacity: 0 },
                  ],
                  {
                    duration: 200,
                  },
                );
                a.finished.then(done);
              }}
            >
              <For each={state.downloadCompleted}>
                {(item) => (
                  <DownloadItem item={item} onRemove={handleRemoveCompleted} />
                )}
              </For>
            </TransitionGroup>
          </Section>
        </div>
      </div>
    </>
  );
};

export default Downloads;
