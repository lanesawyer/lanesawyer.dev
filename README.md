# lanesawyer.dev

Lane Sawyer's site and blog, built on [EmDash](https://docs.emdashcms.com/) (Astro) and hosted on Fly.io.

## Running locally

```bash
pnpm install
pnpm dev
```

Open http://localhost:4321/_emdash/admin and complete the setup wizard. Local content lives in `data.db` and `uploads/` (both gitignored). The scaffolder writes `EMDASH_ENCRYPTION_KEY` to `.env`; generate a new one with `npx emdash secrets generate`.

## Deployment

Fly.io runs a single machine with SQLite and media on the `emdash_data` volume mounted at `/data`. EmDash serializes its database and storage config at build time, so the Dockerfile sets `DATABASE_PATH` and `UPLOADS_DIR` in the build stage. Runtime secrets (`EMDASH_ENCRYPTION_KEY`) go in `fly secrets`.

## AI tooling

- `AGENTS.md` (loaded by `CLAUDE.md`) describes the template and its rules.
- `.agents/skills/` holds EmDash's agent skills (`.claude/skills` links to it). Update with `npx skills update`.
- `.mcp.json` connects the EmDash docs MCP server.
