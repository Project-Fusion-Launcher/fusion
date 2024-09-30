import { Button, Dialog, Select } from "@repo/ui";
import type { Game } from "../routes/Library";
import { createResource, createEffect, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

interface InstallDialogProps {
  selectedGame: Game | null;
  open: boolean;
  handleDialogClose: () => void;
}

interface GameVersion {
  id: string;
  gameId: string;
  source: string;
  name: string;
  downloadSize: number;
}

const InstallDialog = (props: InstallDialogProps) => {
  const [versions, { refetch }] = createResource<GameVersion[]>(fetchVersions);
  const [selectedVersion, setSelectedVersion] =
    createSignal<GameVersion | null>();

  async function fetchVersions(): Promise<GameVersion[]> {
    if (props.selectedGame === null) return [];
    const versions = (await invoke("fetch_game_versions", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
    }).catch(() => [])) as GameVersion[];
    if (versions.length === 1) setSelectedVersion(versions[0]);
    return versions;
  }

  createEffect(() => {
    if (props.selectedGame) {
      setSelectedVersion(null);
      refetch();
    }
  });

  return (
    <Dialog
      title={
        "Install" +
        (props.selectedGame ? ` ${props.selectedGame?.title}` : "Game")
      }
      defaultOpen
      open={props.open}
      onOpenChange={props.handleDialogClose}
    >
      <div class="mb-40 flex gap-40">
        <img
          src="https://cdn.cloudflare.steamstatic.com/steam/apps/2835570/library_600x900_2x.jpg?t=1723031183"
          class="w-192 rounded"
        />
        <Select
          variant="outline"
          placeholder={
            versions.loading
              ? "Fetching versions"
              : (versions()?.length || 0) === 0
                ? "No versions available"
                : "Select a version"
          }
          loading={versions.loading}
          options={
            !versions.loading ? versions()?.map((version) => version.name) : []
          }
          label="Version to install"
          value={selectedVersion()?.name}
          onChange={(value) => {
            setSelectedVersion(
              versions()?.find((version) => version.name === value) || null,
            );
          }}
        />
      </div>
      <div class="flex flex-row-reverse gap-16">
        <Button variant="accent" disabled={selectedVersion() === null}>
          Install
        </Button>
        <Button variant="outline" onClick={props.handleDialogClose}>
          Cancel
        </Button>
      </div>
    </Dialog>
  );
};

export default InstallDialog;
