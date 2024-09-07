interface GameCardProps {
  title: string;
  developer?: string;
}

const GameCard = (props: GameCardProps) => {
  return (
    <div class="w-192 group flex flex-shrink-0 flex-col">
      <div class="bg-secondary h-288 group-hover:outline-accent w-full rounded outline-none outline-2 transition-all group-hover:shadow-[0_0_1.5rem_rgba(255,255,255,0.25)]">
        img goes here
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
