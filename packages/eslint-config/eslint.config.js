import globals from "globals";
import tseslint from "typescript-eslint";
import js from "@eslint/js";
import solid from "eslint-plugin-solid/configs/typescript.js";

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
  js.configs.recommended,
  ...tseslint.configs.recommended,
  solid,
  {
    languageOptions: {
      ecmaVersion: 2020,
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parser: tseslint.parser,
    },
    rules: {
      "@typescript-eslint/consistent-type-imports": "warn",
    },
  },
];
