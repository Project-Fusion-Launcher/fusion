import {
  Boxes,
  Download,
  Joystick,
  Library,
  Store,
  UsersRound,
} from "lucide-solid";
import type { Component } from "solid-js";

export interface Page {
  name: string;
  icon: Component;
}

export const pages = {
  library: { name: "library", icon: Library },
  retro: { name: "retro", icon: Joystick },
  collections: { name: "collections", icon: Boxes },
  storefronts: { name: "storefronts", icon: Store },
  friends: { name: "friends", icon: UsersRound },
  downloads: { name: "downloads", icon: Download },
};
