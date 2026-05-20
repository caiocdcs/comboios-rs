import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { execSync } from "child_process";

function getCommitHash(): string {
  try {
    return execSync("git rev-parse --short HEAD").toString().trim();
  } catch {
    return "dev";
  }
}

export default defineConfig({
  plugins: [sveltekit()],
  define: {
    COMMIT_HASH: JSON.stringify(process.env.COMMIT_HASH || getCommitHash()),
  },
  server: {
    port: 5173,
    strictPort: false,
  },
});
