import { TextField } from "@repo/ui";

interface HeaderProps {
  title: string;
}

const Header = (props: HeaderProps) => {
  return (
    <div class="w-full px-40 py-44">
      <div class="flex items-center gap-40">
        <span class="text-primary w-auto text-4xl font-bold">
          {props.title}
        </span>
        <TextField variant="outline" size="large" placeholder="Search" />
      </div>
    </div>
  );
};

export default Header;
