import { TextField } from "@repo/ui";
import { Search } from "lucide-solid";
import { createSignal } from "solid-js";

interface HeaderProps {
  title: string;
}

const Header = (props: HeaderProps) => {
  const [search, setSearch] = createSignal("");

  return (
    <div class="w-full px-40 py-44">
      <div class="flex grow items-center gap-40">
        <span class="text-primary w-auto text-4xl font-bold">
          {props.title}
        </span>
        <TextField
          variant="outline"
          size="lg"
          placeholder="Search"
          width="full"
          autocomplete="off"
          value={search()}
          onInput={setSearch}
          icon={Search}
        />
      </div>
    </div>
  );
};

export default Header;
