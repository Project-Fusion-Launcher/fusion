import Separator from "./src/components/Separator";
import {
  TextField,
  TextFieldInput,
  TextFieldLabel,
} from "./src/components/TextField";
import Button from "./src/components/Button";
import Badge from "./src/components/Badge";
import {
  Select,
  SelectItem,
  SelectValue,
  SelectContent,
  SelectTrigger,
  SelectHiddenSelect,
  SelectLabel,
} from "./src/components/Select";
import Dialog from "./src/components/Dialog";
import IconButton from "./src/components/IconButton";
import {
  ContextMenuItem,
  ContextMenuSubTrigger,
  ContextMenuContent,
  ContextMenuSubContent,
  ContextMenuSeparator,
} from "./src/components/ContextMenu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "./src/components/Tooltip";
import Tabs from "./src/components/Tabs";

import Itchio from "./src/icons/logos/Itchio";
import LegacyGames from "./src/icons/logos/LegacyGames";

import { defaultConfig } from "tailwind-variants";

defaultConfig.twMerge = true;

export {
  TextField,
  TextFieldInput,
  TextFieldLabel,
  Separator,
  Button,
  Badge,
  Select,
  Dialog,
  IconButton,
  ContextMenuItem,
  ContextMenuSubTrigger,
  ContextMenuContent,
  ContextMenuSubContent,
  ContextMenuSeparator,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  Tabs,
  SelectItem,
  SelectValue,
  SelectContent,
  SelectTrigger,
  SelectHiddenSelect,
  SelectLabel,

  // Icons
  Itchio,
  LegacyGames,
};
