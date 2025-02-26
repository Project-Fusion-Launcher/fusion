import type { JSXElement, ValidComponent } from "solid-js";
import { Match, splitProps, Switch } from "solid-js";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";
import * as ButtonPrimitive from "@kobalte/core/button";
import type { PolymorphicProps } from "@kobalte/core";
import { cn } from "../utils";
import { LoaderCircle } from "lucide-solid";
import { Transition } from "solid-transition-group";

const buttonVariants = tv({
  base: "relative inline-flex items-center justify-center overflow-hidden whitespace-nowrap rounded-md text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
  variants: {
    variant: {
      primary: "bg-primary text-primary-foreground",
      secondary: "bg-secondary text-secondary-foreground",
      accent: "bg-accent text-accent-foreground",
      ghost: "bg-transparent",
      outline: "border-border text-primary border",
    },
    size: {
      sm: "h-32 px-32 text-xs",
      md: "h-40 px-40",
    },
  },
  compoundVariants: [{ variant: "ghost", class: "px-0" }],
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type ButtonProps<T extends ValidComponent = "button"> =
  ButtonPrimitive.ButtonRootProps<T> &
    VariantProps<typeof buttonVariants> & {
      class?: string | undefined;
      children?: JSXElement;
      loading?: boolean;
    };

const Button = <T extends ValidComponent = "button">(
  props: PolymorphicProps<T, ButtonProps<T>>,
) => {
  const [local, others] = splitProps(props as ButtonProps, [
    "variant",
    "size",
    "class",
    "loading",
    "children",
  ]);
  return (
    <ButtonPrimitive.Root
      class={cn(
        buttonVariants({ variant: local.variant, size: local.size }),
        local.class,
      )}
      {...others}
    >
      <Transition
        enterActiveClass="transition-all ease-in duration-150"
        exitActiveClass="transition-all ease-out duration-150"
        enterClass="opacity-0 -translate-y-[100%] scale-95"
        enterToClass="opacity-100 translate-y-0 scale-100"
        exitClass="opacity-100 translate-y-0 scale-100"
        exitToClass="opacity-0 translate-y-[100%] scale-95"
      >
        <Switch>
          <Match when={props.loading}>
            <LoaderCircle class="absolute animate-spin" />
          </Match>
          <Match when={!props.loading}>
            <div class="absolute">{props.children}</div>
          </Match>
        </Switch>
      </Transition>
    </ButtonPrimitive.Root>
  );
};

export default Button;
