import { TextField } from "@repo/ui";
import { Search } from "lucide-solid";

interface HeaderProps {
  title: string;
  query?: string;
  setQuery?: (query: string) => void;
}

const Header = (props: HeaderProps) => {
  return (
    <div class="px-40 py-44">
      <div class="flex grow items-center gap-40">
        <span class="text-primary w-auto text-xl font-bold">{props.title}</span>
        <TextField
          variant="outline"
          size="lg"
          placeholder="Search"
          width="full"
          autocomplete="off"
          value={props.query || ""}
          onInput={props.setQuery}
          icon={Search}
        />
      </div>
    </div>
  );
};

export default Header;
