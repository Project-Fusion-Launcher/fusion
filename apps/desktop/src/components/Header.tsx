import { TextField, TextFieldInput } from "@repo/ui";
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
        <span class="text-primary w-auto text-4xl font-bold">
          {props.title}
        </span>
        <Show when={!props.hideSearch}>
          <TextField
            class="w-full"
            value={props.query}
            onChange={props.onQueryInput}
          >
            <TextFieldInput placeholder="Search" size="lg" autocomplete="off" />
          </TextField>
        </Show>
      </div>
    </div>
  );
};

export default Header;
