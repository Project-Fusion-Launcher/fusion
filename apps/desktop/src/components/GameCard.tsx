interface GameCardProps {
  title: string;
}

const GameCard = (props: GameCardProps) => {
  return (
    <div class="h-288 bg-primary w-192 mb-28 flex-shrink-0 rounded">
      {props.title}
    </div>
  );
};

export default GameCard;
