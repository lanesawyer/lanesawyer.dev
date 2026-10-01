import { LibsqlDialect } from "@libsql/kysely-libsql";

// EmDash serializes database config into the server bundle at build time, so
// the stock libsql() adapter would bake the auth token into the image. This
// entrypoint reads the connection from the environment at runtime instead.
export function createDialect() {
	return new LibsqlDialect({
		url: process.env.TURSO_DATABASE_URL ?? "file:./data.db",
		authToken: process.env.TURSO_AUTH_TOKEN,
	});
}
