import { TextField as KTextField } from "@kobalte/core/text-field";
import type { VariantProps } from "tailwind-variants";
import { tv } from "tailwind-variants";

const variants = tv({
  base: "rounded",
  variants: {
    variant: {
      default: "bg-border",
      outline: "border-border border bg-transparent",
    },
    size: {
      default: "",
      large: "h-48",
    },
  },
  defaultVariants: {
    variant: "default",
    size: "default",
  },
});

type TextFieldVariants = VariantProps<typeof variants>;

export interface TextFieldProps extends TextFieldVariants {
  label?: string;
  placeholder?: string;
}

const TextField = (props: TextFieldProps) => {
  return (
    <KTextField>
      <KTextField.Label>{props.label}</KTextField.Label>
      <KTextField.Input
        placeholder={props.placeholder}
        class={variants({ size: props.size, variant: props.variant })}
      />
    </KTextField>
  );
};

export default TextField;
