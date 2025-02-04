import {
  Boxes,
  Download,
  Globe,
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

export const pages: Record<string, Page> = {
  library: { name: "library", icon: Library },
  retro: { name: "retro", icon: Joystick },
  collections: { name: "collections", icon: Boxes },
  storefronts: { name: "storefronts", icon: Store },
  friends: { name: "friends", icon: UsersRound },
  downloads: { name: "downloads", icon: Download },
  settings: { name: "settings", icon: Settings },
};

export const settingsTabs: Record<string, Page> = {
  storefronts: { name: "storefronts", icon: Store },
  general: { name: "general", icon: Globe },
  downloads: { name: "downloads", icon: Download },
};
