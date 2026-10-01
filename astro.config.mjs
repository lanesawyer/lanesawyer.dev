import node from "@astrojs/node";
import react from "@astrojs/react";
import { defineConfig, fontProviders } from "astro/config";
import emdash, { local, s3 } from "emdash/astro";
import { libsql } from "emdash/db";
import { fileURLToPath } from "node:url";

export default defineConfig({
	output: "server",
	adapter: node({
		mode: "standalone",
	}),
	image: {
		layout: "constrained",
		responsiveStyles: true,
	},
	integrations: [
		react(),
		emdash({
			// The url here only feeds the `emdash migrate` CLI; the server connects via turso.ts.
			database: {
				...libsql({ url: process.env.TURSO_DATABASE_URL ?? "file:./data.db" }),
				entrypoint: fileURLToPath(new URL("./src/db/turso.ts", import.meta.url)),
			},
			// Chosen at build time (EmDash serializes this config into the bundle).
			// s3() reads its S3_* credentials from the environment at runtime.
			storage:
				process.env.MEDIA_STORAGE === "s3"
					? s3()
					: local({
							directory: "./uploads",
							baseUrl: "/_emdash/api/media/file",
						}),
		}),
	],
	fonts: [
		{
			provider: fontProviders.google(),
			name: "Inter",
			cssVariable: "--font-body",
			weights: [400, 500, 600, 700],
			fallbacks: ["sans-serif"],
		},
		{
			provider: fontProviders.google(),
			name: "JetBrains Mono",
			cssVariable: "--font-mono",
			weights: [400, 500],
			fallbacks: ["monospace"],
		},
	],
	devToolbar: { enabled: false },
});
