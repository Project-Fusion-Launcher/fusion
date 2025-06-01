import globals from "globals";
import tseslint from "typescript-eslint";
import js from "@eslint/js";
import plugin from "eslint-plugin-solid";
import { globalIgnores } from "eslint/config";

export default [
  globalIgnores(["**/bindings.ts"]),
  js.configs.recommended,
  ...tseslint.configs.recommended,
  plugin.configs["flat/typescript"],
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
      "@typescript-eslint/consistent-type-imports": [
        "warn",
        { prefer: "type-imports", fixStyle: "separate-type-imports" },
      ],
      "@typescript-eslint/ban-ts-comment": "off",
    },
  },
];
