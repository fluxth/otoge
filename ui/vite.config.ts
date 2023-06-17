import fs from "fs";

import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

import type { ViteDevServer } from "vite";

// Use data from `otoge-generator` for dev server
const dataPlugin = () => ({
  name: "data-preview",
  configureServer(server: ViteDevServer) {
    server.middlewares.use((req, res, next) => {
      if (req.method === "GET" && req.url?.startsWith("/data")) {
        // this is not meant to be secure, don't @ me.
        const path = `../generated${req.url.replace("/data", "")}`;

        if (fs.existsSync(path)) {
          res.setHeader("Content-Type", "application/json");
          const data = fs.readFileSync(path);
          res.end(data);
        } else {
          res.statusCode = 404;
          res.end("Not found");
        }
      } else next();
    });
  },
});

export default defineConfig({
  plugins: [sveltekit(), dataPlugin()],
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },
});
