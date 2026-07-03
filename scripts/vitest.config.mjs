import { defineConfig } from "vitest/config"

export default defineConfig({
  test: {
    environment: "node",
    include: ["scripts/**/*.test.mjs"],
    exclude: ["**/node_modules/**", "**/src-tauri/target/**"],
    clearMocks: true,
    mockReset: true,
    restoreMocks: true,
  },
})
