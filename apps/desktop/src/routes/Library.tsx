import { Badge, Button } from "@repo/ui";
import Header from "../components/Header";

const Library = () => {
  return (
    <>
      <Header title="Library" />
      <div class="h-28 px-40">
        <div class="flex h-full w-min items-center gap-40">
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">All Games</span>
            <Badge variant="accent">100</Badge>
          </Button>
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">Installed</span>
            <Badge variant="outline">69</Badge>
          </Button>
          <Button variant="ghost">
            <span class="text-primary whitespace-nowrap">Not Installed</span>
            <Badge variant="outline">31</Badge>
          </Button>
        </div>
      </div>
    </>
  );
};

export default Library;
