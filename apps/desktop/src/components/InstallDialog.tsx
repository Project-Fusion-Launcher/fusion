import { Button, Dialog, IconButton, Select, TextField } from "@repo/ui";
import {
  createResource,
  createEffect,
  createSignal,
  Match,
  Switch,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { Download, Folder, HardDrive, LoaderCircle } from "lucide-solid";
import { open } from "@tauri-apps/plugin-dialog";
import type { GameVersion, VersionDownloadInfo } from "../models/types";
import { type Game } from "../models/types";
import { bytesToSize } from "../util/string";

interface InstallDialogProps {
  selectedGame: Game | null;
  open: boolean;
  onDialogClose: () => void;
}

const InstallDialog = (props: InstallDialogProps) => {
  const [versions, { refetch: refetchVersions }] =
    createResource(fetchVersions);
  const [selectedVersion, setSelectedVersion] =
    createSignal<GameVersion | null>();

  const [versionDownloadInfo, { refetch: refetchVersionDownloadInfo }] =
    createResource<VersionDownloadInfo | null>(fetchVersionDownloadInfo);

  const [installLocation, setInstallLocation] = createSignal(
    "C:\\Users\\jorge\\Desktop",
  );
  const [preparingToInstall, setPreparingToInstall] = createSignal(false);

  // Fetch the versions of the selected game
  async function fetchVersions(): Promise<GameVersion[]> {
    if (props.selectedGame === null) return [];
    const versions = await invoke<GameVersion[]>("fetch_game_versions", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
    }).catch(() => []);
    if (versions.length === 1) setSelectedVersion(versions[0]);
    return versions;
  }

  // Fetch the download info of the selected version
  async function fetchVersionDownloadInfo(): Promise<VersionDownloadInfo | null> {
    if (selectedVersion() === null) return null;
    return await invoke<VersionDownloadInfo>("fetch_version_info", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
      versionId: selectedVersion()?.id,
    }).catch(() => ({ installSize: 0 }));
  }

  // Refetch the versions when the selected game changes
  createEffect(() => {
    if (props.selectedGame) {
      console.log("Refetching versions");
      setSelectedVersion(null);
      refetchVersions();
    }
  });

  // Handle the selection of a version
  function handleVersionSelect(value: string | null) {
    const version = versions()?.find((v) => v.name === value) || null;
    setSelectedVersion(version);
    if (version && !version.external) {
      refetchVersionDownloadInfo();
    }
  }

  // Open a dialog to select the install location
  function handleDirectorySelect() {
    open({
      multiple: false,
      directory: true,
    }).then((result) => {
      if (result) setInstallLocation(result);
    });
  }

  // Handle the installation of the selected game
  function handleInstall() {
    if (
      selectedVersion() === null ||
      !installLocation() ||
      preparingToInstall()
    )
      return;
    setPreparingToInstall(true);
    invoke("download_game", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
      versionId: selectedVersion()?.id,
      downloadOptions: {
        installLocation: installLocation(),
      },
    }).then(() => {
      handleDialogClose();
      setPreparingToInstall(false);
    });
  }

  // Handle the closing of the dialog
  function handleDialogClose() {
    props.onDialogClose();
  }

  /* HELPER FUNCTIONS */

  // Get the mapped version names
  function getMappedVersions() {
    return versions()?.map((version) => version.name) || [];
  }

  // Get the placeholder for the version select
  function getPlaceholder() {
    if (versions.loading) return "Fetching versions";
    if ((versions()?.length || 0) === 0) return "No versions available";
    return "Select a version";
  }

  return (
    <Dialog
      title={"Install " + props.selectedGame?.title}
      open={props.open}
      onOpenChange={handleDialogClose}
    >
      <div class="mb-40 flex min-w-[300px] gap-32">
        <div class="w-192 h-288 border-border flex shrink-0 overflow-hidden rounded-md border">
          <img
            src={props.selectedGame?.coverUrl}
            class="h-auto w-auto object-cover"
          />
        </div>
        <div class="flex w-full flex-col gap-20">
          <Select
            variant="outline"
            placeholder={getPlaceholder()}
            loading={versions.loading}
            options={getMappedVersions()}
            label="Version to install"
            value={selectedVersion()?.name}
            onChange={handleVersionSelect}
            disallowEmptySelection
          />
          <div class="flex items-end gap-8">
            <TextField
              value={installLocation()}
              onInput={setInstallLocation}
              variant="outline"
              label="Install location"
              placeholder="C:\Users\jorge\Desktop"
            />
            <IconButton variant="outline" onClick={handleDirectorySelect}>
              <Folder />
            </IconButton>
          </div>
          <table
            class="text-secondary flex-cole flex text-base"
            classList={{ "opacity-0": !selectedVersion() }}
          >
            <tbody>
              <tr>
                <td class="flex items-center gap-8 pr-16 font-light">
                  <Download class="size-16" /> Download size:
                </td>
                <td>
                  {selectedVersion()?.external
                    ? "Unknown"
                    : bytesToSize(selectedVersion()?.downloadSize)}
                </td>
              </tr>
              <tr>
                <td class="flex items-center gap-8 pr-16 font-light">
                  <HardDrive class="size-16" />
                  Install size:
                </td>
                <td>
                  <Switch>
                    <Match when={selectedVersion()?.external}>Unknown</Match>
                    <Match when={versionDownloadInfo.loading}>
                      <LoaderCircle class="size-16 animate-spin" />
                    </Match>
                    <Match when={!versionDownloadInfo.loading}>
                      {bytesToSize(versionDownloadInfo()?.installSize)}
                    </Match>
                  </Switch>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="flex flex-row-reverse gap-8">
        <Button
          variant="accent"
          disabled={selectedVersion() === null || !installLocation()}
          onClick={handleInstall}
          loading={preparingToInstall()}
        >
          Install
        </Button>
        <Button
          variant="outline"
          onClick={handleDialogClose}
          disabled={preparingToInstall()}
        >
          Cancel
        </Button>
      </div>
    </Dialog>
  );
};

export default InstallDialog;
