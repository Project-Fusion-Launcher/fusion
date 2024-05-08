import { TextField as KTextField } from "@kobalte/core/text-field";
import { tv } from "tailwind-variants";

export interface TextFieldProps {
  variant?: "default" | "outline";
  size?: "default" | "large";
}

const variants = tv({
  variants: {
    variant: {
      default: "bg-border",
      outline: "border",
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

const TextField = (props: TextFieldProps) => {
  return (
    <KTextField>
      <KTextField.Label />
      <KTextField.Input
        class={variants({ size: props.size, variant: props.variant })}
      />
    </KTextField>
  );
};

export default TextField;
