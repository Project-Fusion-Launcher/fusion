import { Badge, Button } from "@repo/ui";
import Header from "../components/Header";
import { invoke } from "@tauri-apps/api/core";

const Library = () => {
  const refreshGames = () => {
    invoke("fetch_games").then((res) => {
      console.log(res);
    });
  };

  return (
    <>
      <Header title="Library" />
      <div class="h-28 px-40">
        <div class="flex h-full w-min items-center gap-40">
          <button onClick={refreshGames}>refresh</button>
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
