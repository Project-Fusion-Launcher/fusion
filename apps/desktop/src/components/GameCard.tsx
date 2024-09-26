import { ArrowDownToLine } from "lucide-solid";

interface GameCardProps {
  title: string;
  developer?: string;
  onClick?: () => void;
}

const GameCard = (props: GameCardProps) => {
  return (
    <div
      class="w-192 group flex flex-shrink-0 flex-col hover:cursor-pointer"
      onClick={() => props.onClick && props.onClick()}
    >
      <div class="bg-secondary h-288 group-hover:outline-accent relative w-full overflow-hidden rounded outline-none outline-2 transition-all group-hover:shadow-[0_0_1.5rem_rgba(255,255,255,0.25)]">
        <img
          class="absolute object-cover"
          src="https://thumbnails.pcgamingwiki.com/7/74/Mindustry_-_cover.jpg/400px-Mindustry_-_cover.jpg"
        />
        <div class="bg-background absolute z-10 flex h-full w-full items-center justify-center bg-opacity-60 opacity-0 backdrop-blur-sm transition-opacity group-hover:opacity-100">
          <ArrowDownToLine class="stroke-primary size-1/3" />
        </div>
      </div>
      <span class="text-primary mb-8 mt-16 w-full overflow-hidden text-ellipsis text-nowrap font-medium">
        {props.title}
      </span>
      <span class="text-secondary w-full overflow-hidden text-ellipsis text-nowrap text-[14px] font-medium leading-[14px]">
        {props.developer}
      </span>
    </div>
  );
};

export default GameCard;
