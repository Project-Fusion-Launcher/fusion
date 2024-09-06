interface GameCardProps {
  title: string;
}

const GameCard = (props: GameCardProps) => {
  return (
    <div class="w-192 mb-28 flex flex-shrink-0 flex-col">
      <div class="bg-primary h-288 w-full rounded">img goes here</div>
      <span class="text-primary font-medium">{props.title}</span>
    </div>
  );
};

export default GameCard;
