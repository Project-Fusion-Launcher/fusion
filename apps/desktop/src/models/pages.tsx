import {
  Boxes,
  Download,
  Joystick,
  Library,
  Settings,
  Store,
  UsersRound,
} from "lucide-solid";
import type { Component } from "solid-js";

export interface Page {
  name: string;
  icon: Component<{
    class?: string;
    classList?: Record<string, boolean | undefined>;
  }>;
}

export const pages = {
  library: { name: "library", icon: Library },
  retro: { name: "retro", icon: Joystick },
  collections: { name: "collections", icon: Boxes },
  storefronts: { name: "storefronts", icon: Store },
  friends: { name: "friends", icon: UsersRound },
  downloads: { name: "downloads", icon: Download },
  settings: { name: "settings", icon: Settings },
};
