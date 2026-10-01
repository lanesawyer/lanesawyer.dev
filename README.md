# lanesawyer.dev

Lane Sawyer's site and blog, built on [EmDash](https://docs.emdashcms.com/) (Astro) and hosted on Fly.io.

## Running locally

```bash
pnpm install
pnpm dev
```

Open http://localhost:4321/_emdash/admin and complete the setup wizard. Local content lives in `data.db` and `uploads/` (both gitignored). The scaffolder writes `EMDASH_ENCRYPTION_KEY` to `.env`; generate a new one with `npx emdash secrets generate`.

## Deployment

Fly.io runs the app statelessly: the database is on Turso and media is in a private Tigris bucket, served through `/_emdash/api/media/file`.

EmDash serializes its database and storage config into the server bundle at build time. To keep the Turso token out of the image, `src/db/turso.ts` replaces the stock libSQL entrypoint and reads `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN` at runtime (falling back to `file:./data.db` locally). The Dockerfile sets `MEDIA_STORAGE=s3` in the build stage; `s3()` reads its credentials at runtime. Locally, media goes to `./uploads`.

Runtime secrets go in `fly secrets`:

- `TURSO_DATABASE_URL`, `TURSO_AUTH_TOKEN`
- `S3_ENDPOINT` (`https://fly.storage.tigris.dev`), `S3_BUCKET`, `S3_ACCESS_KEY_ID`, `S3_SECRET_ACCESS_KEY`, `S3_REGION` (`auto`)
- `EMDASH_ENCRYPTION_KEY`

## AI tooling

- `AGENTS.md` (loaded by `CLAUDE.md`) describes the template and its rules.
- `.agents/skills/` holds EmDash's agent skills (`.claude/skills` links to it). Update with `npx skills update`.
- `.mcp.json` connects the EmDash docs MCP server.
