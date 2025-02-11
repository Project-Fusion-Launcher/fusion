import { IconButton, TextField, TextFieldInput } from "@repo/ui";
import { X } from "lucide-solid";
import { Show } from "solid-js";

interface HeaderProps {
  title: string;
  hideSearch?: boolean;
  query?: string;
  onQueryInput?: (query: string) => void;
}

const Header = (props: HeaderProps) => {
  function clearQuery() {
    props.onQueryInput?.("");
    ref.value = "";
    ref.focus();
  }

  let ref: HTMLInputElement;

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
            <div class="relative flex">
              <TextFieldInput
                placeholder="Search"
                size="lg"
                autocomplete="off"
                class="pr-44"
                ref={(el) => (ref = el)}
              />
              <Show when={props.query}>
                <IconButton
                  variant="ghost"
                  class="absolute right-0"
                  size="lg"
                  onClick={clearQuery}
                >
                  <X />
                </IconButton>
              </Show>
            </div>
          </TextField>
        </Show>
      </div>
    </div>
  );
};

export default Header;
