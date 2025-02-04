import { TextField } from "@repo/ui";
import { Search } from "lucide-solid";
import { Show } from "solid-js";

interface HeaderProps {
  title: string;
  hideSearch?: boolean;
  query?: string;
  onQueryInput?: (query: string) => void;
}

const Header = (props: HeaderProps) => {
  return (
    <div class="px-40 py-44">
      <div class="flex h-48 grow items-center gap-40">
        <span class="text-primary w-auto text-xl font-bold">{props.title}</span>
        <Show when={!props.hideSearch}>
          <TextField
            variant="outline"
            size="lg"
            placeholder="Search"
            width="full"
            autocomplete="off"
            value={props.query || ""}
            onInput={props.onQueryInput}
            icon={Search}
          />
        </Show>
      </div>
    </div>
  );
};

export default Header;
