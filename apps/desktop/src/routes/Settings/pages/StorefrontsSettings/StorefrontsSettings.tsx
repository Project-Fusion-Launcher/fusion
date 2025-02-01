import { Itchio, LegacyGames } from "@repo/ui";
import GridItem from "./GridItem";

const StorefrontsSettings = () => {
  return (
    <div class="flex gap-16">
      <GridItem icon={Itchio} name="itchio" />
      <GridItem icon={LegacyGames} name="Legacy Games" />
    </div>
  );
};

export default StorefrontsSettings;
