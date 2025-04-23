import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  IconButton,
  Select,
  SelectContent,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
  TextField,
  TextFieldInput,
  TextFieldLabel,
} from "@repo/ui";
import {
  createResource,
  createEffect,
  createSignal,
  Match,
  Switch,
} from "solid-js";
import { Download, Folder, HardDrive, LoaderCircle } from "lucide-solid";
import { open } from "@tauri-apps/plugin-dialog";
import type { GameVersion, GameVersionInfo } from "../models/types";
import { type Game } from "../models/types";
import { bytesToSize } from "../utils/string";
import {
  downloadGame,
  fetchGameVersionInfo,
  fetchGameVersions,
} from "../services/game";

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

  const [gameVersionInfo, { refetch: refetchGameVersionInfo }] =
    createResource<GameVersionInfo | null>(fetchVersionInfo);

  const [installLocation, setInstallLocation] = createSignal(
    "C:\\Users\\jorge\\Desktop",
  );
  const [preparingToInstall, setPreparingToInstall] = createSignal(false);

  async function fetchVersions(): Promise<GameVersion[]> {
    if (!props.selectedGame) return [];
    const versions = await fetchGameVersions(props.selectedGame).catch(
      () => [],
    );
    if (versions.length === 1) setSelectedVersion(versions[0]);
    return versions;
  }

  async function fetchVersionInfo(): Promise<GameVersionInfo | null> {
    const version = selectedVersion();
    if (!props.selectedGame || !version) return null;
    return await fetchGameVersionInfo(props.selectedGame, version).catch(
      () => ({ installSize: 0, downloadSize: 0 }),
    );
  }

  createEffect(() => {
    if (props.selectedGame) {
      setSelectedVersion(null);
      refetchVersions();
    }
  });

  function handleVersionSelect(version: GameVersion | null) {
    setSelectedVersion(version);
    if (version && !version.external) {
      refetchGameVersionInfo();
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
    const version = selectedVersion();
    const location = installLocation();

    if (!props.selectedGame || !version || !location || preparingToInstall())
      return;
    setPreparingToInstall(true);

    downloadGame(props.selectedGame, version, {
      installLocation: location,
    }).then(() => {
      handleDialogClose();
      setPreparingToInstall(false);
    });
  }

  function handleDialogClose() {
    props.onDialogClose();
  }

  function getPlaceholder() {
    if (versions.loading)
      return (
        <span class="flex h-full items-center gap-8">
          <LoaderCircle class="size-16 animate-spin" />
          Fetching versions
        </span>
      );
    if ((versions()?.length || 0) === 0) return "No versions available";
    return "Select a version";
  }

  return (
    <Dialog open={props.open} onOpenChange={handleDialogClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Install Game</DialogTitle>
          <DialogDescription>{props.selectedGame?.title}</DialogDescription>
        </DialogHeader>
        <div class="mb-40 flex min-w-[300px] gap-32">
          <div class="w-192 h-288 border-border flex shrink-0 overflow-hidden rounded-md border">
            <img
              src={props.selectedGame?.coverUrl}
              class="h-auto w-auto object-cover"
            />
          </div>
          <div class="flex w-full flex-col gap-20">
            <Select<GameVersion>
              placeholder={getPlaceholder()}
              options={versions() || []}
              optionValue="name"
              optionTextValue="name"
              disabled={versions.loading}
              value={selectedVersion()}
              onChange={handleVersionSelect}
              disallowEmptySelection
              itemComponent={(props) => (
                <SelectItem item={props.item}>
                  {props.item.rawValue.name}
                </SelectItem>
              )}
            >
              <SelectLabel>Version to install</SelectLabel>
              <SelectTrigger aria-label="Version">
                <SelectValue<GameVersion> class="text-sm">
                  {(state) => state.selectedOption()?.name}
                </SelectValue>
              </SelectTrigger>
              <SelectContent />
            </Select>
            <div class="flex w-full items-end gap-8">
              <TextField
                value={installLocation()}
                onChange={setInstallLocation}
                class="w-full"
              >
                <TextFieldLabel>Install location</TextFieldLabel>
                <TextFieldInput placeholder="C:\Users\jorge\Desktop" />
              </TextField>
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
                  <td class="text-primary font-medium">
                    <Switch>
                      <Match when={selectedVersion()?.external}>Unknown</Match>
                      <Match when={gameVersionInfo.loading}>
                        <LoaderCircle class="size-16 animate-spin" />
                      </Match>
                      <Match when={!gameVersionInfo.loading}>
                        {bytesToSize(gameVersionInfo()?.downloadSize)}
                      </Match>
                    </Switch>
                  </td>
                </tr>
                <tr>
                  <td class="flex items-center gap-8 pr-16 font-light">
                    <HardDrive class="size-16" />
                    Install size:
                  </td>
                  <td class="text-primary font-medium">
                    <Switch>
                      <Match when={selectedVersion()?.external}>Unknown</Match>
                      <Match when={gameVersionInfo.loading}>
                        <LoaderCircle class="size-16 animate-spin" />
                      </Match>
                      <Match when={!gameVersionInfo.loading}>
                        {bytesToSize(gameVersionInfo()?.installSize)}
                      </Match>
                    </Switch>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <DialogFooter class="gap-8">
          <Button
            variant="accent"
            disabled={selectedVersion() === null || !installLocation()}
            onClick={handleInstall}
            loading={preparingToInstall()}
            class="w-[170px]"
          >
            Install
          </Button>
          <Button
            variant="outline"
            onClick={handleDialogClose}
            disabled={preparingToInstall()}
            class="w-[170px]"
          >
            Cancel
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default InstallDialog;
