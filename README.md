# lanesawyer.dev

Lane Sawyer's site and blog, built on [EmDash](https://docs.emdashcms.com/) (Astro) and hosted on Fly.io.

## Running locally

```bash
pnpm install
pnpm dev
```

Open http://localhost:4321/_emdash/admin and complete the setup wizard. Local content lives in `data.db` and `uploads/` (both gitignored). The scaffolder writes `EMDASH_ENCRYPTION_KEY` to `.env`; generate a new one with `npx emdash secrets generate`.

## Deployment

Fly.io runs a single machine with the database on Turso and media on the `emdash_data` volume mounted at `/data`.

EmDash serializes its database and storage config into the server bundle at build time. To keep the Turso token out of the image, `src/db/turso.ts` replaces the stock libSQL entrypoint and reads `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN` at runtime (falling back to `file:./data.db` locally). Storage has no such hook, so the Dockerfile sets `UPLOADS_DIR` in the build stage.

Runtime secrets go in `fly secrets`: `TURSO_DATABASE_URL`, `TURSO_AUTH_TOKEN`, `EMDASH_ENCRYPTION_KEY`.

## AI tooling

- `AGENTS.md` (loaded by `CLAUDE.md`) describes the template and its rules.
- `.agents/skills/` holds EmDash's agent skills (`.claude/skills` links to it). Update with `npx skills update`.
- `.mcp.json` connects the EmDash docs MCP server.
