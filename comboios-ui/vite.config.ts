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

// Mirror the nginx proxy in the UI Dockerfile so `bun run dev` and
// `bun run preview` can reach the API with the same relative URLs.
const apiProxy = Object.fromEntries(
  ["/stations", "/trains", "/ping", "/diagnostics", "/refresh"].map((path) => [
    path,
    process.env.API_URL || "http://localhost:3000",
  ]),
);

export default defineConfig({
  plugins: [sveltekit()],
  define: {
    COMMIT_HASH: JSON.stringify(process.env.COMMIT_HASH || getCommitHash()),
  },
  server: {
    port: 5173,
    strictPort: false,
    proxy: apiProxy,
  },
  preview: {
    proxy: apiProxy,
  },
});
