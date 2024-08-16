import { capitalizeFirstLetter } from "../util/string";

interface HeaderProps {
  title: string;
}

const Header = (props: HeaderProps) => {
  return (
    <div class="w-full px-40 py-44">
      <div class="flex items-center gap-40">
        <span class="text-primary w-auto text-4xl font-bold transition-all">
          {capitalizeFirstLetter(props.title)}
        </span>
        <input class="h-48 w-full transition-all" type="text" />
      </div>
    </div>
  );
};

export default Header;
