import { Button } from "@kobalte/core/button";
import type { Component } from "solid-js";
import { Dynamic } from "solid-js/web";

interface GridItemProps {
  icon?: Component<{ class?: string }>;
  name: string;
}

const GridItem = (props: GridItemProps) => {
  return (
    <Button class="border-border border-sm text-primary flex h-[152px] w-[152px] flex-col items-center justify-center gap-8 rounded bg-gray-200 p-4">
      <Dynamic component={props.icon} class="fill-primary size-72" />
      {props.name}
    </Button>
  );
};

export default GridItem;
