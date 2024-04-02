module.exports = {
  env: {
    browser: true,
    node: true,
  },
  parser: "@typescript-eslint/parser",
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:solid/typescript",
    "plugin:prettier/recommended",
  ],
  plugins: ["@typescript-eslint", "solid"],
  parserOptions: {
    ecmaVersion: 2020,
  },
  rules: {
    "@typescript-eslint/consistent-type-imports": "warn",
    "prettier/prettier": [
      "warn",
      {
        endOfLine: "auto",
        singleQuote: false,
        useTabs: false,
        tabWidth: 2,
        semi: true,
      },
    ],
  },
};
