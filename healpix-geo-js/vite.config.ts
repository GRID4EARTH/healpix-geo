/// <reference types="vitest" />

import { defineConfig } from "vite";

export default defineConfig({
  test: {
    globals: true,
    /// disabled because it causes vitest to error on Symbol.dispose in the wasm-bindgen generated code
    // typecheck: {
    //   enabled: true,
    // },
  },
});
