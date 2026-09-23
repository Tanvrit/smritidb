// Shared flat ESLint config for the Smritidb monorepo.
//
// Every package's `lint` script (`eslint …`) resolves this via a one-line
// `eslint.config.mjs` in the package that re-exports this array. Rules are
// deliberately the non-type-checked recommended sets: the type-aware pass is
// already covered by `pnpm typecheck` (tsc --noEmit) in CI, and running
// typescript-eslint's type-checked configs would duplicate that cost.

import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
  {
    ignores: [
      "**/dist/**",
      "**/build/**",
      "**/node_modules/**",
      "**/.next/**",
      "**/out/**",
      "**/target/**",
      "**/*.d.ts",
    ],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    languageOptions: {
      ecmaVersion: 2023,
      sourceType: "module",
      globals: {
        ...globals.node,
        ...globals.browser,
      },
    },
    rules: {
      // Unused args are meaningful documentation on adapter/interface
      // implementations; require the conventional leading underscore instead
      // of banning them.
      "@typescript-eslint/no-unused-vars": [
        "error",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_", caughtErrors: "none" },
      ],
    },
  },
);
