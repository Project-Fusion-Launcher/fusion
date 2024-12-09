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
import { type Game } from "../models/types";
import { bytesToSize } from "../util/string";

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

interface VersionDownloadInfo {
  installSize: number;
}

const InstallDialog = (props: InstallDialogProps) => {
  const [versions, { refetch: refetchVersions }] =
    createResource<GameVersion[]>(fetchVersions);

  const [selectedVersion, setSelectedVersion] =
    createSignal<GameVersion | null>();
  const [versionDownloadInfo, { refetch: refetchVersionDownloadInfo }] =
    createResource<VersionDownloadInfo | null>(fetchVersionDownloadInfo);
  const [installLocation, setInstallLocation] = createSignal<string>(
    "C:\\Users\\jorge\\Desktop",
  );
  const [preparingToInstall, setPreparingToInstall] = createSignal(false);

  async function fetchVersions(): Promise<GameVersion[]> {
    if (props.selectedGame === null) return [];
    const versions = (await invoke("fetch_game_versions", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
    }).catch(() => [])) as GameVersion[];
    if (versions.length === 1) setSelectedVersion(versions[0]);
    return versions;
  }

  async function fetchVersionDownloadInfo(): Promise<VersionDownloadInfo | null> {
    if (selectedVersion() === null) return null;
    return (await invoke("fetch_version_info", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
      versionId: selectedVersion()?.id,
    }).catch(() => ({ installSize: 0 }))) as VersionDownloadInfo;
  }

  createEffect(() => {
    if (props.selectedGame) {
      setSelectedVersion(null);
      refetchVersions();
    }
  });

  function handleVersionSelect(value: string | null) {
    const version =
      versions()?.find((version) => version.name === value) || null;
    setSelectedVersion(version);
    if (version) {
      refetchVersionDownloadInfo();
    }
  }

  function handleDirectorySelect() {
    open({
      multiple: false,
      directory: true,
    }).then((result) => {
      if (result) setInstallLocation(result);
    });
  }

  function handleInstall() {
    if (selectedVersion() === null || !installLocation()) return;
    setPreparingToInstall(true);
    invoke("download_game", {
      gameId: props.selectedGame?.id,
      gameSource: props.selectedGame?.source,
      versionId: selectedVersion()?.id,
      downloadOptions: {
        installLocation: installLocation(),
      },
      // eslint-disable-next-line solid/reactivity
    }).then(() => {
      props.handleDialogClose();
      setPreparingToInstall(false);
    });
  }

  function getMappedVersions() {
    return versions()?.map((version) => version.name) || [];
  }

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
      <div class="mb-40 flex min-w-[300px] gap-40">
        <div class="w-192 h-288 border-border flex flex-shrink-0 overflow-hidden rounded border">
          <img
            src={props.selectedGame?.coverUrl}
            class="h-auto w-auto object-cover"
          />
        </div>
        <div class="flex w-full flex-col gap-20">
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
              <Folder class="size-16" />
            </IconButton>
          </div>

          <table
            class="text-secondary flex-cole text-md flex"
            classList={{ "opacity-0": !selectedVersion() }}
          >
            <tbody>
              <tr>
                <td class="flex items-center gap-8 pr-16 font-light">
                  <Download class="size-16" /> Download size:
                </td>
                <td>{bytesToSize(selectedVersion()?.downloadSize)}</td>
              </tr>
              <tr>
                <td class="flex items-center gap-8 pr-16 font-light">
                  <HardDrive class="size-16" />
                  Install size:
                </td>
                <td>
                  <Switch>
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
          onClick={props.handleDialogClose}
          disabled={preparingToInstall()}
        >
          Cancel
        </Button>
      </div>
    </Dialog>
  );
};

export default InstallDialog;
